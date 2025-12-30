// Generated macro for CertCompressor (trait)
macro_rules! Depcrate_compressCertCompressor {
() => {
// Module: crate::compress
// Provides: {"CertCompressor"}
// Dependencies: {}
# [doc = " An available certificate compression algorithm."] pub trait CertCompressor : Debug + Send + Sync { # [doc = " Compress `input`, returning the result."] # [doc = ""] # [doc = " `input` is consumed by this function so (if the underlying implementation"] # [doc = " supports it) the compression can be performed in-place."] # [doc = ""] # [doc = " `level` is a hint as to how much effort to expend on the compression."] # [doc = ""] # [doc = " `Err(CompressionFailed)` may be returned for any reason."] fn compress (& self , input : Vec < u8 > , level : CompressionLevel ,) -> Result < Vec < u8 > , CompressionFailed > ; # [doc = " Which algorithm this compressor handles."] fn algorithm (& self) -> CertificateCompressionAlgorithm ; }
};
}
