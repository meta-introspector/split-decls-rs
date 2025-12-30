// Generated macro for join_randoms (function)
macro_rules! Depcrate_tls12join_randoms {
() => {
// Module: crate::tls12
// Provides: {"join_randoms"}
// Dependencies: {}
fn join_randoms (first : & [u8 ; 32] , second : & [u8 ; 32]) -> [u8 ; 64] { let mut randoms = [0u8 ; 64] ; randoms [.. 32] . copy_from_slice (first) ; randoms [32 ..] . copy_from_slice (second) ; randoms }
};
}
