// Generated macro for Check (enum)
macro_rules! Depcrate_streamCheck {
() => {
// Module: crate::stream
// Provides: {"Check"}
// Dependencies: {}
# [doc = " Possible integrity checks that can be part of a .xz stream."] # [allow (missing_docs)] # [derive (Copy , Clone)] pub enum Check { None = lzma_sys :: LZMA_CHECK_NONE as isize , Crc32 = lzma_sys :: LZMA_CHECK_CRC32 as isize , Crc64 = lzma_sys :: LZMA_CHECK_CRC64 as isize , Sha256 = lzma_sys :: LZMA_CHECK_SHA256 as isize , }
};
}
