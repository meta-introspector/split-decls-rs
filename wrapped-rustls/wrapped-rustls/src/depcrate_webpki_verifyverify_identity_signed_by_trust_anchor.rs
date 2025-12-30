// Generated macro for verify_identity_signed_by_trust_anchor (function)
macro_rules! Depcrate_webpki_verifyverify_identity_signed_by_trust_anchor {
() => {
// Module: crate::webpki::verify
// Provides: {"verify_identity_signed_by_trust_anchor"}
// Dependencies: {}
# [doc = " Verify that the end-entity certificate `end_entity` is a valid server cert"] # [doc = " and chains to at least one of the trust anchors in the `roots` [RootCertStore]."] # [doc = ""] # [doc = " This function is primarily useful when building a custom certificate verifier. It"] # [doc = " performs **no revocation checking**. Implementers must handle this themselves,"] # [doc = " along with checking that the server certificate is valid for the subject name"] # [doc = " being used (see [`verify_server_name`])."] # [doc = ""] # [doc = " `intermediates` contains all certificates other than `end_entity` that"] # [doc = " were sent as part of the server's `Certificate` message. It is in the"] # [doc = " same order that the server sent them and may be empty."] pub fn verify_identity_signed_by_trust_anchor (cert : & ParsedCertificate < '_ > , roots : & RootCertStore , intermediates : & [CertificateDer < '_ >] , now : UnixTime , supported_algs : & [& dyn SignatureVerificationAlgorithm] ,) -> Result < () , Error > { verify_identity_signed_by_trust_anchor_impl (cert , roots , intermediates , None , now , supported_algs ,) }
};
}
