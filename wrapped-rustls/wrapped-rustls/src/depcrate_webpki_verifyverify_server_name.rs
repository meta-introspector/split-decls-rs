// Generated macro for verify_server_name (function)
macro_rules! Depcrate_webpki_verifyverify_server_name {
() => {
// Module: crate::webpki::verify
// Provides: {"verify_server_name"}
// Dependencies: {}
# [doc = " Verify that the `end_entity` has an alternative name matching the `server_name`."] # [doc = ""] # [doc = " Note: this only verifies the name and should be used in conjunction with more verification"] # [doc = " like [verify_identity_signed_by_trust_anchor]"] pub fn verify_server_name (cert : & ParsedCertificate < '_ > , server_name : & ServerName < '_ > ,) -> Result < () , Error > { cert . 0 . verify_is_valid_for_subject_name (server_name) . map_err (pki_error) }
};
}
