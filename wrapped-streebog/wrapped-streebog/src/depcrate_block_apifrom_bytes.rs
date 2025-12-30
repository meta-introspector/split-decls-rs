// Generated macro for from_bytes (function)
macro_rules! Depcrate_block_apifrom_bytes {
() => {
// Module: crate::block_api
// Provides: {"from_bytes"}
// Dependencies: {}
# [inline (always)] fn from_bytes (b : & Block) -> [u64 ; 8] { let mut t = [0u64 ; 8] ; for (v , chunk) in t . iter_mut () . zip (b . chunks_exact (8)) { * v = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } t }
};
}
