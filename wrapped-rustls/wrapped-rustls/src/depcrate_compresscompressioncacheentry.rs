// Generated macro for CompressionCacheEntry (struct)
macro_rules! Depcrate_compressCompressionCacheEntry {
() => {
// Module: crate::compress
// Provides: {"CompressionCacheEntry"}
// Dependencies: {}
# [cfg_attr (not (feature = "std") , expect (dead_code))] # [derive (Debug)] pub (crate) struct CompressionCacheEntry { algorithm : CertificateCompressionAlgorithm , original : Vec < u8 > , compressed : CompressedCertificatePayload < 'static > , }
};
}
