use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A compiler query. `query ... { ... }`"] struct Query { doc_comments : Vec < Attribute > , modifiers : QueryModifiers , name : Ident , key : Pat , arg : Type , result : ReturnType , }
}