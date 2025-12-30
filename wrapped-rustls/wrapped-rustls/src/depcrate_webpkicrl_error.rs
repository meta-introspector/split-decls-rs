// Generated macro for crl_error (function)
macro_rules! Depcrate_webpkicrl_error {
() => {
// Module: crate::webpki
// Provides: {"crl_error"}
// Dependencies: {}
fn crl_error (e : webpki :: Error) -> CertRevocationListError { use webpki :: Error :: * ; match e { InvalidCrlSignatureForPublicKey => CertRevocationListError :: BadSignature , UnsupportedCrlSignatureAlgorithm (cx) => { CertRevocationListError :: UnsupportedSignatureAlgorithm { signature_algorithm_id : cx . signature_algorithm_id , supported_algorithms : cx . supported_algorithms , } } UnsupportedCrlSignatureAlgorithmForPublicKey (cx) => { CertRevocationListError :: UnsupportedSignatureAlgorithmForPublicKey { signature_algorithm_id : cx . signature_algorithm_id , public_key_algorithm_id : cx . public_key_algorithm_id , } } InvalidCrlNumber => CertRevocationListError :: InvalidCrlNumber , InvalidSerialNumber => CertRevocationListError :: InvalidRevokedCertSerialNumber , IssuerNotCrlSigner => CertRevocationListError :: IssuerInvalidForCrl , MalformedExtensions | BadDer | BadDerTime => CertRevocationListError :: ParseError , UnsupportedCriticalExtension => CertRevocationListError :: UnsupportedCriticalExtension , UnsupportedCrlVersion => CertRevocationListError :: UnsupportedCrlVersion , UnsupportedDeltaCrl => CertRevocationListError :: UnsupportedDeltaCrl , UnsupportedIndirectCrl => CertRevocationListError :: UnsupportedIndirectCrl , UnsupportedRevocationReason => CertRevocationListError :: UnsupportedRevocationReason , _ => CertRevocationListError :: Other (OtherError :: new (e)) , } }
};
}
