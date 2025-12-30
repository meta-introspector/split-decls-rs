// Generated macro for compress_slice_ng (function)
macro_rules! Depcrate_helperscompress_slice_ng {
() => {
// Module: crate::helpers
// Provides: {"compress_slice_ng"}
// Dependencies: {}
pub fn compress_slice_ng < 'a > (output : & 'a mut [u8] , input : & [u8] , config : DeflateConfig ,) -> (& 'a mut [u8] , ReturnCode) { compress_slice_with_flush_ng (output , input , config , DeflateFlush :: Finish) }
};
}
