// Generated macro for compress (function)
macro_rules! Depcrate_deflatecompress {
() => {
// Module: crate::deflate
// Provides: {"compress"}
// Dependencies: {}
pub fn compress < 'a > (output : & 'a mut [MaybeUninit < u8 >] , input : & [u8] , config : DeflateConfig ,) -> (& 'a mut [u8] , ReturnCode) { compress_with_flush (output , input , config , DeflateFlush :: Finish) }
};
}
