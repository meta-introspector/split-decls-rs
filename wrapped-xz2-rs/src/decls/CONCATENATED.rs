macro_rules! CONCATENATED {
    () => {
        # [doc = " A flag passed when initializing a decoder, indicates that the stream may be"] # [doc = " multiple concatenated xz files."] pub const CONCATENATED : u32 = lzma_sys :: LZMA_CONCATENATED ;
    };
}

CONCATENATED!()