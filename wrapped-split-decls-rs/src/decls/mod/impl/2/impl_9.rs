use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < N : Idx + Ord , const BR : bool > Successors for VecGraph < N , BR > { fn successors (& self , node : N) -> impl Iterator < Item = Self :: Node > { self . successors (node) . iter () . cloned () } }