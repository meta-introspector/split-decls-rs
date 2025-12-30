// Generated macro for adler32_fold_copy (function)
macro_rules! Depcrate_adler32adler32_fold_copy {
() => {
// Module: crate::adler32
// Provides: {"adler32_fold_copy"}
// Dependencies: {}
pub fn adler32_fold_copy (start_checksum : u32 , dst : & mut [u8] , src : & [u8]) -> u32 { debug_assert ! (dst . len () >= src . len () , "{} < {}" , dst . len () , src . len ()) ; dst [.. src . len ()] . copy_from_slice (src) ; adler32 (start_checksum , src) }
};
}
