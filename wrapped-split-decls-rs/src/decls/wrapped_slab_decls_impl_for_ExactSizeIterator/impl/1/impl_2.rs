use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ExactSizeIterator for Drain < '_ , T > { fn len (& self) -> usize { self . len } }
}