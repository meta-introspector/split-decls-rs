macro_rules! TELL_UNSUPPORTED_CHECK {
    () => {
        # [doc = " A flag passed when initializing a decoder, causes `process` to return"] # [doc = " `Error::UnsupportedCheck` if the stream being decoded has an integrity check"] # [doc = " that cannot be verified by this build of liblzma."] pub const TELL_UNSUPPORTED_CHECK : u32 = lzma_sys :: LZMA_TELL_UNSUPPORTED_CHECK ;
    };
}

TELL_UNSUPPORTED_CHECK!()