use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn walk_flat_map_pat_field < T : MutVisitor > (vis : & mut T , mut fp : PatField ,) -> SmallVec < [PatField ; 1] > { vis . visit_pat_field (& mut fp) ; smallvec ! [fp] }