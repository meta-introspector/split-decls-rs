// Generated macro for CertificateStore (struct)
macro_rules! Depcrate_verification_windowsCertificateStore {
() => {
// Module: crate::verification::windows
// Provides: {"CertificateStore"}
// Dependencies: {}
# [doc = " An in-memory Windows certificate store."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `CertificateStore` creates `Certificate` objects that may outlive the"] # [doc = " `CertificateStore`. This is only safe to do if the certificate store is"] # [doc = " constructed with `CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG`."] struct CertificateStore { inner : NonNull < c_void > , engine : Option < CertEngine > , }
};
}
