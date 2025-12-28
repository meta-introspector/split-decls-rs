use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > MixedBitSet < T > { pub fn domain_size (& self) -> usize { match self { MixedBitSet :: Small (set) => set . domain_size () , MixedBitSet :: Large (set) => set . domain_size () , } } }
}