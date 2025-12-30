// Generated macro for invalid_certificate (function)
macro_rules! Depcrate_verificationinvalid_certificate {
() => {
// Module: crate::verification
// Provides: {"invalid_certificate"}
// Dependencies: {}
# [cfg (any (windows , target_vendor = "apple"))] fn invalid_certificate (reason : impl Into < String >) -> rustls :: Error { rustls :: Error :: InvalidCertificate (rustls :: CertificateError :: Other (rustls :: OtherError (Arc :: from (Box :: from (reason . into ())) ,))) }
};
}
