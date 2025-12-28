use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub (crate) struct Interner (Lock < InternerInner >) ;
}