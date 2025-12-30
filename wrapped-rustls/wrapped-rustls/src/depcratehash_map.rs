// Generated macro for hash_map (module)
macro_rules! Depcratehash_map {
() => {
// Module: crate
// Provides: {"hash_map"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "hashbrown"))] mod hash_map { # [cfg (feature = "std")] pub (crate) use std :: collections :: HashMap ; # [cfg (feature = "std")] pub (crate) use std :: collections :: hash_map :: Entry ; # [cfg (all (not (feature = "std") , feature = "hashbrown"))] pub (crate) use hashbrown :: HashMap ; # [cfg (all (not (feature = "std") , feature = "hashbrown"))] pub (crate) use hashbrown :: hash_map :: Entry ; }
};
}
