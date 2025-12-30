// Generated macro for to_bytes (function)
macro_rules! Depcrate_block_apito_bytes {
() => {
// Module: crate::block_api
// Provides: {"to_bytes"}
// Dependencies: {}
# [inline (always)] fn to_bytes (b : & [u64 ; 8]) -> Block { let mut t = [0 ; 64] ; for (chunk , v) in t . chunks_exact_mut (8) . zip (b . iter ()) { chunk . copy_from_slice (& v . to_le_bytes ()) ; } t }
};
}
