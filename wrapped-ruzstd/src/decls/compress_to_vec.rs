macro_rules! deps {
    () => {
        CompressionLevel!();
        Read!();
    };
}

macro_rules! compress_to_vec {
    () => {
        deps!();
        # [doc = " Convenience function to compress some source into a Vec without reusing any resources of the compressor"] # [doc = " ```rust"] # [doc = " use ruzstd::encoding::{compress_to_vec, CompressionLevel};"] # [doc = " let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];"] # [doc = " let compressed = compress_to_vec(data, CompressionLevel::Fastest);"] # [doc = " ```"] pub fn compress_to_vec < R : Read > (source : R , level : CompressionLevel) -> Vec < u8 > { let mut vec = Vec :: new () ; compress (source , & mut vec , level) ; vec }
    };
}

compress_to_vec!();