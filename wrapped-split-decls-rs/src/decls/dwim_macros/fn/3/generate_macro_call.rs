use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn generate_macro_call (macro_def : & MacroDefinition , intent : & DwimIntent) -> TokenStream { match macro_def . name . as_str () { "mkbootstrap" => { quote ! { mkbootstrap ! { tools : ["wrap_single_crate" , "bootstrap"] , config : "split-decls-rs.toml" , workspace : true , recursive : true } } . into () } , "mkbuildrs" => { quote ! { mkbuildrs ! { dependencies : { syn = "2.0" , quote = "1.0" } , logic : dwim_build_logic ! () } } . into () } , _ => { quote ! { compile_error ! ("Unknown macro generation") ; } . into () } } }