use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T > Copy for Interned < 'a , T > { }
}