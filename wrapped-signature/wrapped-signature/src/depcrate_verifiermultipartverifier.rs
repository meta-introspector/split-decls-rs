// Generated macro for MultipartVerifier (trait)
macro_rules! Depcrate_verifierMultipartVerifier {
() => {
// Module: crate::verifier
// Provides: {"MultipartVerifier"}
// Dependencies: {}
# [doc = " Equivalent of [`Verifier`] but the message is provided in non-contiguous byte slices."] pub trait MultipartVerifier < S > { # [doc = " Equivalent of [`Verifier::verify()`] but the"] # [doc = " message is provided in non-contiguous byte slices."] fn multipart_verify (& self , msg : & [& [u8]] , signature : & S) -> Result < () , Error > ; }
};
}
