use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents information about a single macro."] # [derive (Debug , Serialize , Deserialize)] struct MacroInfo { name : String , kind : String , file : String , line : usize , column : usize , # [serde (skip_serializing_if = "Option::is_none")] signature : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] doc_comment : Option < String > , }
}