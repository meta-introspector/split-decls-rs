// Generated macro for log_server_cert (function)
macro_rules! Depcrate_verificationlog_server_cert {
() => {
// Module: crate::verification
// Provides: {"log_server_cert"}
// Dependencies: {}
fn log_server_cert (_end_entity : & rustls :: pki_types :: CertificateDer < '_ >) { # [cfg (feature = "cert-logging")] { use base64 :: Engine ; log :: debug ! ("verifying certificate: {}" , base64 :: engine :: general_purpose :: STANDARD . encode (_end_entity . as_ref ())) ; } }
};
}
