use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: compute_dependency_depth");
fn compute_dependency_depth (start : usize , graph : & DependencyGraph) -> usize { let mut visited = HashSet :: new () ; let mut queue = VecDeque :: new () ; let mut max_depth = 0 ; queue . push_back ((start , 0)) ; visited . insert (start) ; while let Some ((node , depth)) = queue . pop_front () { max_depth = max_depth . max (depth) ; if let Some (deps) = graph . forward . get (& node) { for & dep in deps { if ! visited . contains (& dep) { visited . insert (dep) ; queue . push_back ((dep , depth + 1)) ; } } } } max_depth }
}