use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ImplementType { type_name : String , generics : Vec < ImplementType > , # [doc = " The best span for diagnostics."] span : proc_macro2 :: Span , }
}