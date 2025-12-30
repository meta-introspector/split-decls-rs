// Generated macro for hash (function)
macro_rules! Depcrate_hashhash {
() => {
// Module: crate::hash
// Provides: {"hash"}
// Dependencies: {}
pub (crate) fn hash < T : Hash > (t : & T) -> u64 { FxHasher :: default () . hash_one (t) }
};
}
