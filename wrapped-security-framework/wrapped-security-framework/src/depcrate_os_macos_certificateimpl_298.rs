// Generated macro for impl_298 (impl)
macro_rules! Depcrate_os_macos_certificateimpl_298 {
() => {
// Module: crate::os::macos::certificate
// Provides: {"impl_298"}
// Dependencies: {}
impl CertificateProperties { # [doc = " Retrieves a specific property identified by its OID."] # [must_use] pub fn get (& self , oid : CertificateOid) -> Option < CertificateProperty > { unsafe { self . 0 . find (oid . as_ptr () . to_void ()) . map (| value | CertificateProperty (CFDictionary :: wrap_under_get_rule (* value as * mut _))) } } }
};
}
