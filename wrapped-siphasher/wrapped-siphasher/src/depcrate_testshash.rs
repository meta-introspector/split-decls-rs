// Generated macro for hash (function)
macro_rules! Depcrate_testshash {
() => {
// Module: crate::tests
// Provides: {"hash"}
// Dependencies: {}
fn hash < T : Hash > (x : & T) -> u64 { hash_with (SipHasher :: new () , x) }
};
}
