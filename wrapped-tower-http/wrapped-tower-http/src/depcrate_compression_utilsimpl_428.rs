// Generated macro for impl_428 (impl)
macro_rules! Depcrate_compression_utilsimpl_428 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_428"}
// Dependencies: {}
# [cfg (any (feature = "compression-br" , feature = "compression-gzip" , feature = "compression-deflate" , feature = "compression-zstd"))] impl CompressionLevel { pub (crate) fn into_async_compression (self) -> AsyncCompressionLevel { match self { CompressionLevel :: Fastest => AsyncCompressionLevel :: Fastest , CompressionLevel :: Best => AsyncCompressionLevel :: Best , CompressionLevel :: Default => AsyncCompressionLevel :: Default , CompressionLevel :: Precise (quality) => AsyncCompressionLevel :: Precise (quality) , } } }
};
}
