// Generated macro for impl_314 (impl)
macro_rules! Depcrate_os_macos_certificate_oidsimpl_314 {
() => {
// Module: crate::os::macos::certificate_oids
// Provides: {"impl_314"}
// Dependencies: {}
# [allow (missing_docs)] impl CertificateOid { # [inline (always)] # [must_use] pub fn x509_v1_signature_algorithm () -> Self { unsafe { Self (kSecOIDX509V1SignatureAlgorithm) } } # [doc = " Returns the underlying raw pointer corresponding to this OID."] # [inline (always)] # [must_use] pub fn as_ptr (& self) -> CFStringRef { self . 0 } # [doc = " Returns the string representation of the OID."] # [inline] # [must_use] pub fn to_str (& self) -> CFString { unsafe { CFString :: wrap_under_get_rule (self . 0) } } }
};
}
