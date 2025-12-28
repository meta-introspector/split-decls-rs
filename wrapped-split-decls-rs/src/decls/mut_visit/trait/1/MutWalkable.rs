use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait MutWalkable < V : MutVisitor > { fn walk_mut (& mut self , visitor : & mut V) ; }