// Generated macro for verify_identity_signed_by_trust_anchor_impl (function)
macro_rules! Depcrate_webpki_verifyverify_identity_signed_by_trust_anchor_impl {
() => {
// Module: crate::webpki::verify
// Provides: {"verify_identity_signed_by_trust_anchor_impl"}
// Dependencies: {}
# [doc = " Verify that the end-entity certificate `end_entity` is a valid server cert"] # [doc = " and chains to at least one of the trust anchors in the `roots` [RootCertStore]."] # [doc = ""] # [doc = " `intermediates` contains all certificates other than `end_entity` that"] # [doc = " were sent as part of the server's `Certificate` message. It is in the"] # [doc = " same order that the server sent them and may be empty."] # [doc = ""] # [doc = " `revocation` controls how revocation checking is performed, if at all."] # [doc = ""] # [doc = " This function exists to be used by [`verify_identity_signed_by_trust_anchor`],"] # [doc = " and differs only in providing a `Option<webpki::RevocationOptions>` argument. We"] # [doc = " can't include this argument in `verify_identity_signed_by_trust_anchor` because"] # [doc = " it will leak the webpki types into Rustls' public API."] pub (crate) fn verify_identity_signed_by_trust_anchor_impl (cert : & ParsedCertificate < '_ > , roots : & RootCertStore , intermediates : & [CertificateDer < '_ >] , revocation : Option < webpki :: RevocationOptions < '_ > > , now : UnixTime , supported_algs : & [& dyn SignatureVerificationAlgorithm] ,) -> Result < () , Error > { let result = cert . 0 . verify_for_usage (supported_algs , & roots . roots , intermediates , now , & ExtendedKeyUsage :: server_auth () , revocation , None ,) ; match result { Ok (_) => Ok (()) , Err (e) => Err (pki_error (e)) , } }
};
}
