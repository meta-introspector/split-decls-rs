// Generated macro for impl_79 (impl)
macro_rules! Depcrate_cert_storeimpl_79 {
() => {
// Module: crate::cert_store
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a > Iterator for Certs < 'a > { type Item = CertContext ; fn next (& mut self) -> Option < CertContext > { unsafe { let cur = self . cur . take () . map (| p | { let ptr = p . as_inner () ; mem :: forget (p) ; ptr }) ; let cur = cur . unwrap_or (ptr :: null_mut ()) ; let next = Cryptography :: CertEnumCertificatesInStore (self . store . 0 , cur) ; if next . is_null () { self . cur = None ; None } else { let next = CertContext :: from_inner (next) ; self . cur = Some (next . clone ()) ; Some (next) } } } }
};
}
