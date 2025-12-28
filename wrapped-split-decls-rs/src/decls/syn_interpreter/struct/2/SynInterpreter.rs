use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Syn-based interpreter for executing split-decls-rs functions"] pub struct SynInterpreter { functions : HashMap < String , ItemFn > , variables : HashMap < String , SynValue > , call_stack : Vec < String > , }
}