// Generated macro for default_cert_decompressors (function)
macro_rules! Depcrate_compressdefault_cert_decompressors {
() => {
// Module: crate::compress
// Provides: {"default_cert_decompressors"}
// Dependencies: {}
# [doc = " Returns the supported `CertDecompressor` implementations enabled"] # [doc = " by crate features."] pub fn default_cert_decompressors () -> & 'static [& 'static dyn CertDecompressor] { & [# [cfg (feature = "brotli")] BROTLI_DECOMPRESSOR , # [cfg (feature = "zlib")] ZLIB_DECOMPRESSOR ,] }
};
}
