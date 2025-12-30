// Generated macro for hash (function)
macro_rules! Depcrate_tests128hash {
() => {
// Module: crate::tests128
// Provides: {"hash"}
// Dependencies: {}
fn hash < T : Hash > (x : & T) -> [u8 ; 16] { hash_with (SipHasher :: new () , x) }
};
}
