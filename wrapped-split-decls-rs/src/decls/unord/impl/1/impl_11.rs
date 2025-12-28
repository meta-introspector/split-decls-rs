use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > UnordItems < T , std :: iter :: Empty < T > > { pub fn empty () -> Self { UnordItems (std :: iter :: empty ()) } }