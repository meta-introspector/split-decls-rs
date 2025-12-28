macro_rules! IGNORE_CHECK {
    () => {
        # [doc = " A flag passed when initializing a decoder, causes the decoder to ignore any"] # [doc = " integrity checks listed."] pub const IGNORE_CHECK : u32 = lzma_sys :: LZMA_TELL_UNSUPPORTED_CHECK ;
    };
}

IGNORE_CHECK!();