use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Macro-callable binary system - call any extracted program via macros"] # [derive (Parser)] # [command (name = "macro-runner")] # [command (about = "Run extracted programs via macro calls")] struct Args { # [doc = " Macro to execute"] # [arg (short , long)] macro_name : String , # [doc = " Arguments for the macro"] # [arg (trailing_var_arg = true)] args : Vec < String > , }
}