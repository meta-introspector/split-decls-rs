// Generated macro for hash_once (function)
macro_rules! Depcratehash_once {
() => {
// Module: crate
// Provides: {"hash_once"}
// Dependencies: {}
pub fn hash_once < Hasher : std :: hash :: Hasher + Default > (thing : impl std :: hash :: Hash) -> u64 { std :: hash :: BuildHasher :: hash_one (& std :: hash :: BuildHasherDefault :: < Hasher > :: default () , thing) }
};
}
