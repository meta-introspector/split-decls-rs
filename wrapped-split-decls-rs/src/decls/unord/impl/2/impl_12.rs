use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : Clone + 'a , I : Iterator < Item = & 'a T > > UnordItems < & 'a T , I > { # [inline] pub fn cloned (self) -> UnordItems < T , impl Iterator < Item = T > > { UnordItems (self . 0 . cloned ()) } }