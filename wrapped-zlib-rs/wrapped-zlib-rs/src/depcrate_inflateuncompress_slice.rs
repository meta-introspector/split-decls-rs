// Generated macro for uncompress_slice (function)
macro_rules! Depcrate_inflateuncompress_slice {
() => {
// Module: crate::inflate
// Provides: {"uncompress_slice"}
// Dependencies: {}
pub fn uncompress_slice < 'a > (output : & 'a mut [u8] , input : & [u8] , config : InflateConfig ,) -> (& 'a mut [u8] , ReturnCode) { let output_uninit = unsafe { core :: slice :: from_raw_parts_mut (output . as_mut_ptr () as * mut MaybeUninit < u8 > , output . len ()) } ; uncompress (output_uninit , input , config) }
};
}
