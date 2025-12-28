use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Default for Slab < T > { fn default () -> Self { Slab :: new () } }
}