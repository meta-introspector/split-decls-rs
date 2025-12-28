use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (crate) trait Walkable < 'a , V : Visitor < 'a > > { # [must_use] fn walk_ref (& 'a self , visitor : & mut V) -> V :: Result ; }