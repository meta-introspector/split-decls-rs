use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Embedded RDFa/SHACL/OWL ontology for macro semantics"] const MACRO_ONTOLOGY : & str = r#"
@prefix dwim: <http://split-decls.rs/ontology/dwim#> .
@prefix macro: <http://split-decls.rs/ontology/macro#> .

dwim:mkbootstrap a macro:DeclarativeMacro ;
    macro:purpose "Generate bootstrap infrastructure" ;
    macro:inputs ( macro:ToolList macro:ConfigFile macro:WorkspaceFlag ) ;
    macro:outputs ( macro:BootstrapScript macro:Makefile macro:CargoToml ) .

dwim:mkbuildrs a macro:DeclarativeMacro ;
    macro:purpose "Generate self-contained build.rs" ;
    macro:inputs ( macro:Dependencies macro:BuildLogic ) ;
    macro:outputs ( macro:BuildScript ) .
"# ;