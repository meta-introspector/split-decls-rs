// Generated macro for impl_1619 (impl)
macro_rules! Depcrate_errorimpl_1619 {
() => {
// Module: crate::error
// Provides: {"impl_1619"}
// Dependencies: {}
impl PartialEq < Self > for CertRevocationListError { fn eq (& self , other : & Self) -> bool { use CertRevocationListError :: * ; match (self , other) { (BadSignature , BadSignature) => true , (UnsupportedSignatureAlgorithm { signature_algorithm_id : left_signature_algorithm_id , supported_algorithms : left_supported_algorithms , } , UnsupportedSignatureAlgorithm { signature_algorithm_id : right_signature_algorithm_id , supported_algorithms : right_supported_algorithms , } ,) => { (left_signature_algorithm_id , left_supported_algorithms) == (right_signature_algorithm_id , right_supported_algorithms) } (UnsupportedSignatureAlgorithmForPublicKey { signature_algorithm_id : left_signature_algorithm_id , public_key_algorithm_id : left_public_key_algorithm_id , } , UnsupportedSignatureAlgorithmForPublicKey { signature_algorithm_id : right_signature_algorithm_id , public_key_algorithm_id : right_public_key_algorithm_id , } ,) => { (left_signature_algorithm_id , left_public_key_algorithm_id) == (right_signature_algorithm_id , right_public_key_algorithm_id) } (InvalidCrlNumber , InvalidCrlNumber) => true , (InvalidRevokedCertSerialNumber , InvalidRevokedCertSerialNumber) => true , (IssuerInvalidForCrl , IssuerInvalidForCrl) => true , (ParseError , ParseError) => true , (UnsupportedCrlVersion , UnsupportedCrlVersion) => true , (UnsupportedCriticalExtension , UnsupportedCriticalExtension) => true , (UnsupportedDeltaCrl , UnsupportedDeltaCrl) => true , (UnsupportedIndirectCrl , UnsupportedIndirectCrl) => true , (UnsupportedRevocationReason , UnsupportedRevocationReason) => true , _ => false , } } }
};
}
