/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use buck2_core::target::label::ConfiguredTargetLabel;
use buck2_interpreter_for_build::attrs::coerce::query_functions::QUERY_FUNCTIONS;
use buck2_node::nodes::configured::ConfiguredTargetNode;
use buck2_node::nodes::configured_ref::ConfiguredGraphNodeRef;
use buck2_node::nodes::configured_ref::ConfiguredGraphNodeRefLookup;
use buck2_query::query::compatibility::MaybeCompatible;
use buck2_query::query::environment::deps;
use buck2_query::query::environment::LabeledNode;
use buck2_query::query::environment::QueryEnvironment;
use buck2_query::query::environment::QueryTarget;
use buck2_query::query::environment::TraversalFilter;
use buck2_query::query::syntax::simple::eval::error::QueryError;
use buck2_query::query::syntax::simple::eval::file_set::FileSet;
use buck2_query::query::syntax::simple::eval::set::TargetSet;
use buck2_query::query::syntax::simple::eval::values::QueryValue;
use buck2_query::query::syntax::simple::functions::helpers::QueryBinaryOp;
use buck2_query::query::syntax::simple::functions::helpers::QueryFunction;
use buck2_query::query::syntax::simple::functions::DefaultQueryFunctionsModule;
use buck2_query::query::syntax::simple::functions::QueryFunctions;
use buck2_query::query::traversal::async_depth_limited_traversal;
use buck2_query::query::traversal::async_fast_depth_first_postorder_traversal;
use buck2_query::query::traversal::AsyncTraversalDelegate;
use buck2_query::query_module;
use buck2_query_parser::BinaryOp;
use dice::DiceComputations;
use dupe::Dupe;
use indexmap::IndexMap;
use inventory::ctor;
use thiserror::Error;

use crate::actions::artifact::artifact_type::Artifact;
use crate::actions::artifact::artifact_type::OutputArtifact;
use crate::analysis::configured_graph::AnalysisConfiguredGraphQueryDelegate;
use crate::analysis::configured_graph::AnalysisDiceQueryDelegate;
use crate::artifact_groups::deferred::DeferredTransitiveSetData;
use crate::artifact_groups::deferred::TransitiveSetKey;
use crate::artifact_groups::ArtifactGroup;
use crate::deferred::types::DeferredValueReady;
use crate::interpreter::rule_defs::artifact_tagging::ArtifactTag;
use crate::interpreter::rule_defs::cmd_args::CommandLineArtifactVisitor;
use crate::interpreter::rule_defs::cmd_args::ValueAsCommandLineLike;
use crate::interpreter::rule_defs::transitive_set::TransitiveSet;

#[derive(Debug, Error)]
pub(crate) enum AnalysisQueryError {
    #[error("file literals aren't supported in query attributes (got `{0}`)")]
    FileLiteralsNotAllowed(String),
    #[error(
        "template_placeholder_info `{0}` of target `{1}` used in query attributes had artifact (`{2}`) not produced by a target, only target-produced artifacts supported here"
    )]
    NonTargetBoundArtifact(String, ConfiguredTargetLabel, Artifact),
    #[error("Internal error: literal `{0}` not found in `deps`")]
    LiteralNotFoundInDeps(String),
}

#[async_trait]
pub trait ConfiguredGraphQueryEnvironmentDelegate: Send + Sync {
    fn eval_literal(&self, literal: &str) -> anyhow::Result<ConfiguredTargetNode>;

    async fn get_template_info_provider_artifacts(
        &self,
        configured_label: &ConfiguredTargetLabel,
        template_name: &str,
    ) -> anyhow::Result<Vec<ArtifactGroup>>;

    /// Looks up a transitive set.
    ///
    /// WE CANNOT USE THIS TO LOOKUP ALL NODES IN A TSET GRAPH!!! Doing so can lead to
    /// massive memory regressions.
    ///
    /// Using this will add dice edges for any fetched keys. It's very important
    /// that we do not flatten a full tset into the dice deps of our analysis nodes.
    ///
    /// Really, we shouldn't need this at all. We have references to the starlark
    /// values and we should be able to traverse them directly, but the cmdlinearg
    /// apis don't allow us to do that (they convert values into ArtifactGroup,
    /// which holds only the symbolic reference to the tset, for example).
    async fn dice_lookup_transitive_set(
        &self,
        key: TransitiveSetKey,
    ) -> anyhow::Result<DeferredValueReady<DeferredTransitiveSetData>>;

    async fn get_from_template_placeholder_info(
        &self,
        template_name: &'static str,
        targets: &TargetSet<ConfiguredGraphNodeRef>,
    ) -> anyhow::Result<IndexMap<ConfiguredTargetLabel, Artifact>> {
        let mut label_to_artifact: IndexMap<ConfiguredTargetLabel, Artifact> = IndexMap::new();

        // Traversing tsets adds complexity here. Ideally, we could just do a normal traversal of these starlark values
        // we get from the template_info provider, but the cmdlinearglike interface only gives us access via ArtifactGroup
        // which only have the key for the tset projection, not the tset or projection itself.
        //
        // Then, next we would want to do a traversal over those ArtifactGroup and use the dice ctx to map the tset projection
        // keys back to their values. We can't do that because that would flatten the tset into the dice deps and rdeps storage.
        //
        // So, instead we need to extract out the ArtifactGroups from the template_info values and lookup the corresponding
        // tsets, manually traverse them and repeat (because the tset node values themselves return ArtifactGroup).
        //
        // This means that we will have unnessary dice nodes pointing to each tset value appearing in the template info and any
        // tset value appearing in another tset's nodes. In the common case, though, that set will be small and we won't have
        // flattened any full tset into the dice deps storage. We'll call those "top-level" tset nodes.

        // This will contain the ArtifactGroups we encounter during our traversal (so only artifacts and top-level tset nodes).
        // Artifacts are put here to keep them in the correct order in the output, tsets are top-level tset nodes that we need
        // to traverse.
        let artifacts = futures::future::try_join_all(targets.iter().map(|target| async move {
            let artifacts = self
                .get_template_info_provider_artifacts(target.label(), template_name)
                .await?;
            anyhow::Ok(
                artifacts
                    .into_iter()
                    .map(|artifact| (target.label().dupe(), artifact)),
            )
        }))
        .await?;
        let mut artifacts: VecDeque<_> = artifacts.into_iter().flatten().collect();

        // This will contain the TransitiveSetProjectionKey we encounter as top-level nodes and we will also put in TransitiveSetProjectionKey
        // for all the tset nodes that we encounter during our traversal of those top-level nodes. We don't need to track artifacts because
        // we just extract the targetlabel and put that in the output set and that can dedupe them (and we don't need to further
        // traverse artifacts).
        let mut seen = HashSet::new();

        while let Some((target, artifact)) = artifacts.pop_front() {
            match artifact {
                ArtifactGroup::Artifact(artifact) => {
                    if let Some(owner) = artifact.owner() {
                        let target_label = owner.unpack_target_label().ok_or_else(|| {
                            AnalysisQueryError::NonTargetBoundArtifact(
                                template_name.to_owned(),
                                target.dupe(),
                                artifact.dupe(),
                            )
                        })?;
                        label_to_artifact.insert(target_label.dupe(), artifact.dupe());
                    }
                }
                ArtifactGroup::TransitiveSetProjection(tset_key) => {
                    // We've encountered a "top-level" tset node that we haven't yet seen (as either a top-level or intermediate node, doesn't matter).
                    if seen.insert(tset_key.dupe()) {
                        let tset_value = self.dice_lookup_transitive_set(tset_key.key).await?;

                        // Now we can traverse this tset from that node. This is a different traversal than our top-level one as we will
                        // be accessing tset internals directly and so we can actually traverse the starlark objects without going back through
                        // dice. We'll be working all with values with lifetimes from `tset_value`.
                        //
                        // We can't use tset's normal traverse because we need to avoid retraversing parts of the tset graph that we've already
                        // traversed (through other top-level tset nodes).
                        let mut queue = VecDeque::new();
                        queue.push_back(tset_value.as_value());
                        while let Some(v) = queue.pop_front() {
                            let as_tset =
                                TransitiveSet::from_value(v).context("invalid tset structure")?;

                            // Visit the projection value itself. As this is an opaque cmdargs-like thing, it may contain more top-level tset node
                            // references that need to be pushed into the outer queue.
                            if let Some(v) = as_tset.get_projection_value(tset_key.projection)? {
                                struct Visitor<'a>(
                                    &'a mut VecDeque<(ConfiguredTargetLabel, ArtifactGroup)>,
                                    ConfiguredTargetLabel,
                                );
                                impl<'a> CommandLineArtifactVisitor for Visitor<'a> {
                                    fn visit_input(
                                        &mut self,
                                        input: ArtifactGroup,
                                        _tag: Option<&ArtifactTag>,
                                    ) {
                                        self.0.push_back((self.1.dupe(), input));
                                    }

                                    fn visit_output(
                                        &mut self,
                                        _artifact: OutputArtifact,
                                        _tag: Option<&ArtifactTag>,
                                    ) {
                                        // ignored
                                    }
                                }
                                v.as_command_line_err()?
                                    .visit_artifacts(&mut Visitor(&mut artifacts, target.dupe()))?;
                            }

                            // Enqueue any children we haven't yet seen (and mark them seen).
                            for child in as_tset.children.iter() {
                                let child_as_tset = TransitiveSet::from_value(*child)
                                    .context("Invalid deferred")?;
                                let projection_key =
                                    child_as_tset.get_projection_key(tset_key.projection);
                                if seen.insert(projection_key) {
                                    queue.push_back(*child);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(label_to_artifact)
    }

    async fn get_targets_from_template_placeholder_info(
        &self,
        template_name: &'static str,
        targets: TargetSet<ConfiguredGraphNodeRef>,
    ) -> anyhow::Result<TargetSet<ConfiguredGraphNodeRef>>;
}

pub struct ConfiguredGraphQueryEnvironment<'a> {
    delegate: &'a dyn ConfiguredGraphQueryEnvironmentDelegate,
}

#[derive(Debug)]
struct ConfiguredGraphFunctions<'a>(PhantomData<&'a ()>);
#[query_module(ConfiguredGraphQueryEnvironment<'a>)]
impl<'a> ConfiguredGraphFunctions<'a> {
    async fn classpath(
        &self,
        env: &ConfiguredGraphQueryEnvironment<'a>,
        targets: TargetSet<ConfiguredGraphNodeRef>,
        depth: Option<u64>,
    ) -> Result<QueryValue<ConfiguredGraphNodeRef>, QueryError> {
        // if depth param is provided and it is not equal to 1, then it's not supported
        let mut run_first_order_classpath = false;
        if let Some(depth_int) = depth.map(|v| v as i32) {
            run_first_order_classpath = depth_int == 1;
            if !run_first_order_classpath {
                return Err(QueryError::InvalidDepth(depth_int));
            }
        }

        let template_name = if run_first_order_classpath {
            "first_order_classpath"
        } else {
            "classpath_including_targets_with_no_output"
        };

        let targets = env
            .get_targets_from_template_placeholder_info(template_name, targets)
            .await?;
        Ok(targets.into())
    }
}

impl<'a> ConfiguredGraphQueryEnvironment<'a> {
    pub fn new(delegate: &'a dyn ConfiguredGraphQueryEnvironmentDelegate) -> Self {
        Self { delegate }
    }

    pub fn functions() -> impl QueryFunctions<Env = ConfiguredGraphQueryEnvironment<'a>> {
        struct Functions<'a> {
            defaults: DefaultQueryFunctionsModule<ConfiguredGraphQueryEnvironment<'a>>,
            extra_functions: ConfiguredGraphFunctions<'a>,
        }

        impl Debug for Functions<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("Functions").finish_non_exhaustive()
            }
        }

        impl<'a> QueryFunctions for Functions<'a> {
            type Env = ConfiguredGraphQueryEnvironment<'a>;
            fn get(
                &self,
                name: &str,
            ) -> Option<&dyn QueryFunction<ConfiguredGraphQueryEnvironment<'a>>> {
                if let Some(v) = self.extra_functions.get(name) {
                    Some(v)
                } else {
                    self.defaults.get(name)
                }
            }

            fn get_op(
                &self,
                op: BinaryOp,
            ) -> Option<&dyn QueryBinaryOp<ConfiguredGraphQueryEnvironment<'a>>> {
                if let Some(v) = self.extra_functions.get_op(op) {
                    Some(v)
                } else {
                    self.defaults.get_op(op)
                }
            }
        }

        Functions {
            defaults: DefaultQueryFunctionsModule::new(),
            extra_functions: ConfiguredGraphFunctions(PhantomData),
        }
    }

    /// For each input target goes into its exposed list of providers, finds TemplatePlaceholderInfo among them
    /// and accesses its internal `keyed_variables` map with the passed `template_name` key.
    /// Then converts retrieved value form cmd args into targets and returns them back as result.
    async fn get_from_template_placeholder_info<'x>(
        &'x self,
        template_name: &'static str,
        targets: &TargetSet<ConfiguredGraphNodeRef>,
    ) -> anyhow::Result<IndexMap<ConfiguredTargetLabel, Artifact>> {
        self.delegate
            .get_from_template_placeholder_info(template_name, targets)
            .await
    }

    async fn get_targets_from_template_placeholder_info(
        &self,
        template_name: &'static str,
        targets: TargetSet<ConfiguredGraphNodeRef>,
    ) -> anyhow::Result<TargetSet<ConfiguredGraphNodeRef>> {
        self.delegate
            .get_targets_from_template_placeholder_info(template_name, targets)
            .await
    }
}

#[ctor]
fn set_query_functions() {
    QUERY_FUNCTIONS.init(Arc::new(ConfiguredGraphQueryEnvironment::functions()));
}

#[async_trait]
impl<'a> QueryEnvironment for ConfiguredGraphQueryEnvironment<'a> {
    type Target = ConfiguredGraphNodeRef;

    async fn get_node(&self, node_ref: &ConfiguredGraphNodeRef) -> anyhow::Result<Self::Target> {
        Ok(node_ref.dupe())
    }

    async fn get_node_for_default_configured_target(
        &self,
        _node_ref: &ConfiguredGraphNodeRef,
    ) -> anyhow::Result<MaybeCompatible<Self::Target>> {
        Err(QueryError::FunctionUnimplemented(
            "get_node_for_default_configured_target() only for CqueryEnvironment",
        )
        .into())
    }

    async fn eval_literals(&self, literal: &[&str]) -> anyhow::Result<TargetSet<Self::Target>> {
        let mut result = TargetSet::new();
        for lit in literal {
            result.insert(ConfiguredGraphNodeRef(self.delegate.eval_literal(lit)?));
        }
        Ok(result)
    }

    async fn eval_file_literal(&self, literal: &str) -> anyhow::Result<FileSet> {
        Err(AnalysisQueryError::FileLiteralsNotAllowed(literal.to_owned()).into())
    }

    async fn dfs_postorder(
        &self,
        root: &TargetSet<Self::Target>,
        delegate: &mut dyn AsyncTraversalDelegate<Self::Target>,
    ) -> anyhow::Result<()> {
        async_fast_depth_first_postorder_traversal(
            &ConfiguredGraphNodeRefLookup,
            root.iter().map(LabeledNode::node_ref),
            delegate,
        )
        .await
    }

    async fn depth_limited_traversal(
        &self,
        root: &TargetSet<Self::Target>,
        delegate: &mut dyn AsyncTraversalDelegate<Self::Target>,
        depth: u32,
    ) -> anyhow::Result<()> {
        async_depth_limited_traversal(&ConfiguredGraphNodeRefLookup, root.iter(), delegate, depth)
            .await
    }

    async fn owner(&self, _paths: &FileSet) -> anyhow::Result<TargetSet<Self::Target>> {
        Err(QueryError::FunctionUnimplemented("owner").into())
    }

    async fn deps(
        &self,
        targets: &TargetSet<Self::Target>,
        depth: Option<i32>,
        filter: Option<&dyn TraversalFilter<Self::Target>>,
    ) -> anyhow::Result<TargetSet<Self::Target>> {
        if filter.is_some() {
            // We fallback to default `deps` implementation here because evaluating filter function is async.
            deps(self, targets, depth, filter).await
        } else {
            // We reimplement `deps` here rather than relying on the default implementation for two reasons.
            // (1) default `deps` implementation is asynchronous, but for configured target nodes, this can be entirely
            // synchronous when a user doesn't specify a filter function (this usually happens in practice).
            // (2) default `deps` currently uses a separate visited target set and result target set, but they can
            // actually be the same - once you've traversed the entire deps graph, the visited set is your result set.
            // This only uses one set and reduces the amount of hashing needed for deps.
            // TODO(scottcao): Make `deps` default implementation use one set for both visited and result targets.
            // TODO(scottcao): Make traversals synchronous for both ConfiguredTargetNode and ConfiguredGraphNodeRef.
            let mut deps = targets.clone();
            let mut queue: VecDeque<_> = targets.iter().map(|target| (target.dupe(), 0)).collect();

            while let Some((target, target_depth)) = queue.pop_front() {
                if depth.map_or(true, |max_depth| max_depth < 0 || target_depth < max_depth) {
                    for dep in target.deps() {
                        // Unlike default `deps` implementation, we just insert into one set here instead of two.
                        if deps.insert(dep.dupe()) {
                            queue.push_back((dep.dupe(), target_depth + 1));
                        }
                    }
                }
            }

            Ok(deps)
        }
    }
}

/// Used by `audit classpath`
pub async fn classpath(
    ctx: &DiceComputations,
    targets: impl Iterator<Item = ConfiguredTargetNode>,
) -> anyhow::Result<IndexMap<ConfiguredTargetLabel, Artifact>> {
    let targets: TargetSet<_> = targets.map(ConfiguredGraphNodeRef).collect();

    let dice_query_delegate = Arc::new(AnalysisDiceQueryDelegate { ctx });
    let delegate = AnalysisConfiguredGraphQueryDelegate {
        dice_query_delegate,
        // Initializing an empty map because we don't have query literals for `audit classpath`. This is a bit hacky
        // TODO(scottcao): Split a classpath delegate out of the configured graph query delegate so we can use it
        // without initializing an empty map.
        resolved_literals: HashMap::new(),
    };
    let env = ConfiguredGraphQueryEnvironment::new(&delegate);
    env.get_from_template_placeholder_info("classpath", &targets)
        .await
}
