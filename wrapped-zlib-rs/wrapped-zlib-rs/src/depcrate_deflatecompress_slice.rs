// Generated macro for compress_slice (function)
macro_rules! Depcrate_deflatecompress_slice {
() => {
// Module: crate::deflate
// Provides: {"compress_slice"}
// Dependencies: {}
pub fn compress_slice < 'a > (output : & 'a mut [u8] , input : & [u8] , config : DeflateConfig ,) -> (& 'a mut [u8] , ReturnCode) { let output_uninit = unsafe { core :: slice :: from_raw_parts_mut (output . as_mut_ptr () as * mut MaybeUninit < u8 > , output . len ()) } ; compress (output_uninit , input , config) }
};
}
