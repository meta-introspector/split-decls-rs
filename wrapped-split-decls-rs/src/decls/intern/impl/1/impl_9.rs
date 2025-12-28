use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T > Clone for Interned < 'a , T > { fn clone (& self) -> Self { * self } }
}