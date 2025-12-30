// Generated macro for Certificate (struct)
macro_rules! Depcrate_verification_windowsCertificate {
() => {
// Module: crate::verification::windows
// Provides: {"Certificate"}
// Dependencies: {}
# [doc = " A representation of a certificate."] # [doc = ""] # [doc = " The `CertificateStore` must be opened with the correct flags to ensure the"] # [doc = " certificate may outlive it; see the `CertificateStore` documentation."] struct Certificate { inner : NonNull < CERT_CONTEXT > , }
};
}
