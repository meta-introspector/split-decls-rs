// Generated macro for compress (function)
macro_rules! Depcrate_compresscompress {
() => {
// Module: crate::compress
// Provides: {"compress"}
// Dependencies: {}
pub (crate) fn compress (state : & mut [u64 ; 8] , blocks : & [[u8 ; 64]]) { for block in blocks { compress_block (state , block) ; } }
};
}
