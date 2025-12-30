// Generated macro for Lzma (enum)
macro_rules! Depcrate_compressionLzma {
() => {
// Module: crate::compression
// Provides: {"Lzma"}
// Dependencies: {}
# [cfg (feature = "lzma")] pub (crate) enum Lzma < R : io :: BufRead > { Uninitialized { reader : Option < R > , uncompressed_size : u64 , } , Initialized (Box < lzma_rust2 :: LzmaReader < R > >) , }
};
}
