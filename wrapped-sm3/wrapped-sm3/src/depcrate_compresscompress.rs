// Generated macro for compress (function)
macro_rules! Depcrate_compresscompress {
() => {
// Module: crate::compress
// Provides: {"compress"}
// Dependencies: {}
pub (crate) fn compress (state : & mut [u32 ; 8] , blocks : & [[u8 ; 64]]) { for block in blocks { let mut w = [0u32 ; 16] ; for (o , chunk) in w . iter_mut () . zip (block . chunks_exact (4)) { * o = u32 :: from_be_bytes (chunk . try_into () . unwrap ()) ; } compress_u32 (state , & w) ; } }
};
}
