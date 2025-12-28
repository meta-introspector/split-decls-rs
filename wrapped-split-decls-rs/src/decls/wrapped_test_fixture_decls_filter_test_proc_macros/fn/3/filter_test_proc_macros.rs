use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn filter_test_proc_macros (proc_macro_names : & [String] , proc_macro_defs : Vec < (String , ProcMacro) > ,) -> (Vec < ProcMacro > , String) { let mut source = String :: new () ; let mut proc_macros = Vec :: new () ; for (c , p) in proc_macro_defs { if ! proc_macro_names . iter () . any (| name | name == & stdx :: to_lower_snake_case (p . name . as_str ())) { continue ; } proc_macros . push (p) ; source += & c ; } (proc_macros , source) }