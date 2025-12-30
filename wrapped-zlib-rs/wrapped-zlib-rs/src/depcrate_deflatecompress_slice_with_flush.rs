// Generated macro for compress_slice_with_flush (function)
macro_rules! Depcrate_deflatecompress_slice_with_flush {
() => {
// Module: crate::deflate
// Provides: {"compress_slice_with_flush"}
// Dependencies: {}
pub fn compress_slice_with_flush < 'a > (output : & 'a mut [u8] , input : & [u8] , config : DeflateConfig , flush : DeflateFlush ,) -> (& 'a mut [u8] , ReturnCode) { let output_uninit = unsafe { core :: slice :: from_raw_parts_mut (output . as_mut_ptr () as * mut MaybeUninit < u8 > , output . len ()) } ; compress_with_flush (output_uninit , input , config , flush) }
};
}
