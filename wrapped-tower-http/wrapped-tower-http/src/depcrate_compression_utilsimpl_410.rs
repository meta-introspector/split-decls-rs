// Generated macro for impl_410 (impl)
macro_rules! Depcrate_compression_utilsimpl_410 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_410"}
// Dependencies: {}
impl SupportedEncodings for AcceptEncoding { # [allow (dead_code)] fn gzip (& self) -> bool { # [cfg (any (feature = "decompression-gzip" , feature = "compression-gzip"))] return self . gzip ; # [cfg (not (any (feature = "decompression-gzip" , feature = "compression-gzip")))] return false ; } # [allow (dead_code)] fn deflate (& self) -> bool { # [cfg (any (feature = "decompression-deflate" , feature = "compression-deflate"))] return self . deflate ; # [cfg (not (any (feature = "decompression-deflate" , feature = "compression-deflate")))] return false ; } # [allow (dead_code)] fn br (& self) -> bool { # [cfg (any (feature = "decompression-br" , feature = "compression-br"))] return self . br ; # [cfg (not (any (feature = "decompression-br" , feature = "compression-br")))] return false ; } # [allow (dead_code)] fn zstd (& self) -> bool { # [cfg (any (feature = "decompression-zstd" , feature = "compression-zstd"))] return self . zstd ; # [cfg (not (any (feature = "decompression-zstd" , feature = "compression-zstd")))] return false ; } }
};
}
