use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Idx for LocalDefId { # [inline] fn new (idx : usize) -> Self { LocalDefId { local_def_index : Idx :: new (idx) } } # [inline] fn index (self) -> usize { self . local_def_index . index () } }
}