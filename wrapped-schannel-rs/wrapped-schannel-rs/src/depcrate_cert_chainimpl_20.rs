// Generated macro for impl_20 (impl)
macro_rules! Depcrate_cert_chainimpl_20 {
() => {
// Module: crate::cert_chain
// Provides: {"impl_20"}
// Dependencies: {}
impl CertChain { # [doc = " Returns the number of certificates in the chain"] pub fn len (& self) -> usize { unsafe { (* self . 0) . cElement as usize } } # [doc = " Returns true if there are no certificates in the chain"] pub fn is_empty (& self) -> bool { unsafe { (* self . 0) . cElement == 0 } } # [doc = " Get the n-th certificate from the current chain"] pub fn get (& self , idx : usize) -> Option < CertContext > { let elements = unsafe { let cert_chain = * self . 0 ; slice :: from_raw_parts (cert_chain . rgpElement as * mut & mut Cryptography :: CERT_CHAIN_ELEMENT , cert_chain . cElement as usize ,) } ; elements . get (idx) . map (| el | { let cert = unsafe { CertContext :: from_inner (el . pCertContext) } ; let rc_cert = cert . clone () ; mem :: forget (cert) ; rc_cert }) } # [doc = " Return an iterator over all certificates in this chain"] pub fn certificates (& self) -> Certificates { Certificates { chain : self , idx : 0 , } } }
};
}
