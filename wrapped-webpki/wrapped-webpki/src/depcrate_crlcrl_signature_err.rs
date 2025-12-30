// Generated macro for crl_signature_err (function)
macro_rules! Depcrate_crlcrl_signature_err {
() => {
// Module: crate::crl
// Provides: {"crl_signature_err"}
// Dependencies: {}
fn crl_signature_err (err : Error) -> Error { match err { Error :: UnsupportedSignatureAlgorithm (cx) => Error :: UnsupportedCrlSignatureAlgorithm (cx) , Error :: UnsupportedSignatureAlgorithmForPublicKey (cx) => { Error :: UnsupportedCrlSignatureAlgorithmForPublicKey (cx) } Error :: InvalidSignatureForPublicKey => Error :: InvalidCrlSignatureForPublicKey , _ => err , } }
};
}
