// Generated macro for TELL_NO_CHECK (const)
macro_rules! Depcrate_streamTELL_NO_CHECK {
() => {
// Module: crate::stream
// Provides: {"TELL_NO_CHECK"}
// Dependencies: {}
# [doc = " A flag passed when initializing a decoder, causes `process` to return"] # [doc = " `Error::NoCheck` if the stream being decoded has no integrity check."] pub const TELL_NO_CHECK : u32 = lzma_sys :: LZMA_TELL_NO_CHECK ;
};
}
