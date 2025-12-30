// Generated macro for xor_block (function)
macro_rules! Depcrate_block_apixor_block {
() => {
// Module: crate::block_api
// Provides: {"xor_block"}
// Dependencies: {}
pub (crate) fn xor_block (state : & mut [u64 ; PLEN] , block : & [u8]) { assert ! (block . len () < 8 * PLEN) ; let mut chunks = block . chunks_exact (8) ; for (s , chunk) in state . iter_mut () . zip (& mut chunks) { * s ^= u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let rem = chunks . remainder () ; if ! rem . is_empty () { let mut buf = [0u8 ; 8] ; buf [.. rem . len ()] . copy_from_slice (rem) ; let n = block . len () / 8 ; state [n] ^= u64 :: from_le_bytes (buf) ; } }
};
}
