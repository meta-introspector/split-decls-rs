// Generated macro for SUPPORTED_COMPRESSION_METHODS (const)
macro_rules! Depcrate_compressionSUPPORTED_COMPRESSION_METHODS {
() => {
// Module: crate::compression
// Provides: {"SUPPORTED_COMPRESSION_METHODS"}
// Dependencies: {}
# [doc = " The compression methods which have been implemented."] pub const SUPPORTED_COMPRESSION_METHODS : & [CompressionMethod] = & [CompressionMethod :: Stored , # [cfg (feature = "_deflate-any")] CompressionMethod :: Deflated , # [cfg (feature = "deflate64")] CompressionMethod :: Deflate64 , # [cfg (feature = "bzip2")] CompressionMethod :: Bzip2 , # [cfg (feature = "zstd")] CompressionMethod :: Zstd , # [cfg (feature = "xz")] CompressionMethod :: Xz , # [cfg (feature = "ppmd")] CompressionMethod :: Ppmd ,] ;
};
}
