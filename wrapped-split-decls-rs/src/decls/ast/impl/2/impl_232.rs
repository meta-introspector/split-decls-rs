use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for FnHeader { fn default () -> FnHeader { FnHeader { safety : Safety :: Default , coroutine_kind : None , constness : Const :: No , ext : Extern :: None , } } }
}