// Generated macro for VerifierBuilderError (enum)
macro_rules! Depcrate_webpkiVerifierBuilderError {
() => {
// Module: crate::webpki
// Provides: {"VerifierBuilderError"}
// Dependencies: {}
# [doc = " An error that can occur when building a certificate verifier."] # [derive (Debug , Clone)] # [non_exhaustive] pub enum VerifierBuilderError { # [doc = " No root trust anchors were provided."] NoRootAnchors , # [doc = " A provided CRL could not be parsed."] InvalidCrl (CertRevocationListError) , }
};
}
