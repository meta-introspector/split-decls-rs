// Generated macro for seed (function)
macro_rules! Depcrate_randseed {
() => {
// Module: crate::rand
// Provides: {"seed"}
// Dependencies: {}
pub fn seed () -> u64 { use std :: hash :: { BuildHasher , Hasher } ; # [allow (clippy :: disallowed_types)] std :: collections :: hash_map :: RandomState :: new () . build_hasher () . finish () }
};
}
