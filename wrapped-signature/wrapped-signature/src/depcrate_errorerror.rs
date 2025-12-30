// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Signature errors."] # [doc = ""] # [doc = " This type is deliberately opaque as to avoid sidechannel leakage which"] # [doc = " could potentially be used recover signing private keys or forge signatures"] # [doc = " (e.g. [BB'06])."] # [doc = ""] # [doc = " When the `alloc` feature is enabled, it supports an optional [`core::error::Error::source`],"] # [doc = " which can be used by things like remote signers (e.g. HSM, KMS) to report I/O or auth errors."] # [doc = ""] # [doc = " [BB'06]: https://en.wikipedia.org/wiki/Daniel_Bleichenbacher"] # [derive (Default)] # [non_exhaustive] pub struct Error { # [doc = " Source of the error (if applicable)."] # [cfg (feature = "alloc")] source : Option < Box < dyn core :: error :: Error + Send + Sync + 'static > > , }
};
}
