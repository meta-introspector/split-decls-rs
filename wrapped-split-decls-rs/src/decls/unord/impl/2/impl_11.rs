use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > UnordItems < T , std :: iter :: Empty < T > > { pub fn empty () -> Self { UnordItems (std :: iter :: empty ()) } }
}