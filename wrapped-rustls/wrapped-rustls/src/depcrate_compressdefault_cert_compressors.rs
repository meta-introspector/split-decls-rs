// Generated macro for default_cert_compressors (function)
macro_rules! Depcrate_compressdefault_cert_compressors {
() => {
// Module: crate::compress
// Provides: {"default_cert_compressors"}
// Dependencies: {}
# [doc = " Returns the supported `CertCompressor` implementations enabled"] # [doc = " by crate features."] pub fn default_cert_compressors () -> & 'static [& 'static dyn CertCompressor] { & [# [cfg (feature = "brotli")] BROTLI_COMPRESSOR , # [cfg (feature = "zlib")] ZLIB_COMPRESSOR ,] }
};
}
