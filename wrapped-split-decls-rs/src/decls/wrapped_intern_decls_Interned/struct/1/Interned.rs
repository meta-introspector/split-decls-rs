use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct Interned < T : Internable + ? Sized > { arc : Arc < T > , }
}