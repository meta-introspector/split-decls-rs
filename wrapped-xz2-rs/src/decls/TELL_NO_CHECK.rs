macro_rules! TELL_NO_CHECK {
    () => {
        # [doc = " A flag passed when initializing a decoder, causes `process` to return"] # [doc = " `Error::NoCheck` if the stream being decoded has no integrity check."] pub const TELL_NO_CHECK : u32 = lzma_sys :: LZMA_TELL_NO_CHECK ;
    };
}

TELL_NO_CHECK!()