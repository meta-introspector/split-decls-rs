use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > DenseBitSet < T > { # [doc = " Gets the domain size."] pub fn domain_size (& self) -> usize { self . domain_size } }
}