use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Default for Arena < T > { fn default () -> Arena < T > { Arena { data : Vec :: new () } } }
}