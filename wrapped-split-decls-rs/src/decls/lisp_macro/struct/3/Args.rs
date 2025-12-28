use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Lisp-like macro system for output2 declarations"] # [derive (Parser)] # [command (name = "lisp-macro")] # [command (about = "Import output2 as macros and interpret them like Lisp")] struct Args { # [doc = " Command to execute"] # [arg (short , long)] command : String , # [doc = " Lisp-like expressions to evaluate"] # [arg (trailing_var_arg = true)] expressions : Vec < String > , }
}