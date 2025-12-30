// Generated macro for CertDecompressor (trait)
macro_rules! Depcrate_compressCertDecompressor {
() => {
// Module: crate::compress
// Provides: {"CertDecompressor"}
// Dependencies: {}
# [doc = " An available certificate decompression algorithm."] pub trait CertDecompressor : Debug + Send + Sync { # [doc = " Decompress `input`, writing the result to `output`."] # [doc = ""] # [doc = " `output` is sized to match the declared length of the decompressed data."] # [doc = ""] # [doc = " `Err(DecompressionFailed)` should be returned if decompression produces more, or fewer"] # [doc = " bytes than fit in `output`, or if the `input` is in any way malformed."] fn decompress (& self , input : & [u8] , output : & mut [u8]) -> Result < () , DecompressionFailed > ; # [doc = " Which algorithm this decompressor handles."] fn algorithm (& self) -> CertificateCompressionAlgorithm ; }
};
}
