use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "std")] impl < K , S > HashSetExt for std :: collections :: HashSet < K , S > where S : BuildHasher + Default , { fn new () -> Self { std :: collections :: HashSet :: with_hasher (S :: default ()) } fn with_capacity (capacity : usize) -> Self { std :: collections :: HashSet :: with_capacity_and_hasher (capacity , S :: default ()) } }
}