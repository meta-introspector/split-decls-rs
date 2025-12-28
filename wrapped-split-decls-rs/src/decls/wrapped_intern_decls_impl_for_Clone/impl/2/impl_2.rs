use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Internable + ? Sized > Clone for Interned < T > { fn clone (& self) -> Self { Self { arc : self . arc . clone () , } } }
}