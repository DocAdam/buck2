"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[861],{3905:(e,t,n)=>{n.r(t),n.d(t,{MDXContext:()=>m,MDXProvider:()=>u,mdx:()=>T,useMDXComponents:()=>c,withMDXComponents:()=>p});var i=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(){return o=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var i in n)Object.prototype.hasOwnProperty.call(n,i)&&(e[i]=n[i])}return e},o.apply(this,arguments)}function r(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);t&&(i=i.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,i)}return n}function d(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?r(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):r(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,i,a=function(e,t){if(null==e)return{};var n,i,a={},o=Object.keys(e);for(i=0;i<o.length;i++)n=o[i],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(i=0;i<o.length;i++)n=o[i],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var m=i.createContext({}),p=function(e){return function(t){var n=c(t.components);return i.createElement(e,o({},t,{components:n}))}},c=function(e){var t=i.useContext(m),n=t;return e&&(n="function"==typeof e?e(t):d(d({},t),e)),n},u=function(e){var t=c(e.components);return i.createElement(m.Provider,{value:t},e.children)},s="mdxType",N={inlineCode:"code",wrapper:function(e){var t=e.children;return i.createElement(i.Fragment,{},t)}},x=i.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,r=e.parentName,m=l(e,["components","mdxType","originalType","parentName"]),p=c(n),u=a,s=p["".concat(r,".").concat(u)]||p[u]||N[u]||o;return n?i.createElement(s,d(d({ref:t},m),{},{components:n})):i.createElement(s,d({ref:t},m))}));function T(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,r=new Array(o);r[0]=x;var d={};for(var l in t)hasOwnProperty.call(t,l)&&(d[l]=t[l]);d.originalType=e,d[s]="string"==typeof e?e:a,r[1]=d;for(var m=2;m<o;m++)r[m]=n[m];return i.createElement.apply(null,r)}return i.createElement.apply(null,n)}x.displayName="MDXCreateElement"},13609:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>r,default:()=>c,frontMatter:()=>o,metadata:()=>d,toc:()=>m});var i=n(87462),a=(n(67294),n(3905));const o={id:"glossary",title:"Glossary"},r=void 0,d={unversionedId:"concepts/glossary",id:"concepts/glossary",title:"Glossary",description:".buckconfig",source:"@site/../docs/concepts/glossary.md",sourceDirName:"concepts",slug:"/concepts/glossary",permalink:"/docs/concepts/glossary",draft:!1,tags:[],version:"current",frontMatter:{id:"glossary",title:"Glossary"},sidebar:"manualSidebar",previous:{title:"Concept Map",permalink:"/docs/concepts/concept_map"},next:{title:"<Temporary Page>",permalink:"/docs/filler"}},l={},m=[{value:".buckconfig",id:"buckconfig",level:4},{value:"Action",id:"action",level:4},{value:"Action graph",id:"action-graph",level:4},{value:"Artifact",id:"artifact",level:4},{value:"Attribute",id:"attribute",level:4},{value:"Build",id:"build",level:4},{value:"Build file",id:"build-file",level:4},{value:"Bxl",id:"bxl",level:4},{value:"Cell",id:"cell",level:4},{value:"Configuration",id:"configuration",level:4},{value:"Configured graph",id:"configured-graph",level:4},{value:"Constraint value",id:"constraint-value",level:4},{value:"Daemon",id:"daemon",level:4},{value:"Dependency",id:"dependency",level:4},{value:"Execution platform",id:"execution-platform",level:4},{value:"Isolation dir",id:"isolation-dir",level:4},{value:"Target pattern",id:"target-pattern",level:4},{value:"Package",id:"package",level:4},{value:"Project",id:"project",level:4},{value:"Provider",id:"provider",level:4},{value:"Remote execution (RE)",id:"remote-execution-re",level:4},{value:"Rule",id:"rule",level:4},{value:"Source file",id:"source-file",level:4},{value:"Starlark",id:"starlark",level:4},{value:"Target",id:"target",level:4},{value:"Toolchain",id:"toolchain",level:4},{value:"Transition",id:"transition",level:4},{value:"Unconfigured graph",id:"unconfigured-graph",level:4},{value:"Visibility",id:"visibility",level:4}],p={toc:m};function c(e){let{components:t,...n}=e;return(0,a.mdx)("wrapper",(0,i.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,a.mdx)("h4",{id:"buckconfig"},".buckconfig"),(0,a.mdx)("p",null,"The root of your ",(0,a.mdx)("a",{parentName:"p",href:"#project"},"project")," must contain a configuration file named ",(0,a.mdx)("inlineCode",{parentName:"p"},".buckconfig"),". Before executing, Buck2 reads this file to incorporate specified customizations. Performs the same role as it did in Buck1. See more: ",(0,a.mdx)("a",{parentName:"p",href:"https://buck2.build/docs/legacy/files-and-directories/dot-buckconfig"},"Legacy docs"),"."),(0,a.mdx)("h4",{id:"action"},"Action"),(0,a.mdx)("p",null,"An individual, cacheable, ideally hermetic command that's run during the ",(0,a.mdx)("a",{parentName:"p",href:"#build"},"build"),". It takes ",(0,a.mdx)("a",{parentName:"p",href:"#artifact"},"artifacts")," as inputs and produces other artifacts as outputs. An example command could be ",(0,a.mdx)("inlineCode",{parentName:"p"},"gcc -o main main.c")," which takes the artifact ",(0,a.mdx)("inlineCode",{parentName:"p"},"main.c")," (a source file) and produces the artifact called ",(0,a.mdx)("inlineCode",{parentName:"p"},"main")," (the compiled binary)."),(0,a.mdx)("h4",{id:"action-graph"},"Action graph"),(0,a.mdx)("p",null,"It's the dependency graph of all the ",(0,a.mdx)("a",{parentName:"p",href:"#action"},"actions")," belonging to a target. Can be queried with ",(0,a.mdx)("inlineCode",{parentName:"p"},"buck2 aquery"),"."),(0,a.mdx)("h4",{id:"artifact"},"Artifact"),(0,a.mdx)("p",null,"A single input or output of an ",(0,a.mdx)("a",{parentName:"p",href:"#action"},"action"),". These are files that participate as inputs or outputs of a build. These can be source files or build outputs. See also: ",(0,a.mdx)("a",{parentName:"p",href:"https://buck2.build/docs/generated/native/Artifact/"},"Artifact API"),"."),(0,a.mdx)("h4",{id:"attribute"},"Attribute"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"build"},"Build"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"build-file"},"Build file"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"bxl"},"Bxl"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"cell"},"Cell"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"configuration"},"Configuration"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"configured-graph"},"Configured graph"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"constraint-value"},"Constraint value"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"daemon"},"Daemon"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"dependency"},"Dependency"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"execution-platform"},"Execution platform"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"isolation-dir"},"Isolation dir"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"target-pattern"},"Target pattern"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"package"},"Package"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"project"},"Project"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"provider"},"Provider"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"remote-execution-re"},"Remote execution (RE)"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"rule"},"Rule"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"source-file"},"Source file"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"starlark"},"Starlark"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"target"},"Target"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"toolchain"},"Toolchain"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"transition"},"Transition"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"unconfigured-graph"},"Unconfigured graph"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")),(0,a.mdx)("h4",{id:"visibility"},"Visibility"),(0,a.mdx)("admonition",{type:"note"},(0,a.mdx)("p",{parentName:"admonition"},"\ud83d\udea7\xa0\xa0\xa0THIS SECTION IS UNDER CONSTRUCTION")))}c.isMDXComponent=!0}}]);