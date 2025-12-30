// Generated macro for impl_18 (impl)
macro_rules! Depcrate_cert_chainimpl_18 {
() => {
// Module: crate::cert_chain
// Provides: {"impl_18"}
// Dependencies: {}
impl CertChainContext { # [doc = " Get the final (for a successful verification this means successful) certificate chain"] # [doc = ""] # [doc = " https://msdn.microsoft.com/de-de/library/windows/desktop/aa377182(v=vs.85).aspx"] # [doc = " rgpChain[cChain - 1] is the final chain"] pub fn final_chain (& self) -> Option < CertChain > { if let Some (chain) = self . chains () . last () { return Some (CertChain (chain . 0 , self . clone ())) ; } None } # [doc = " Retrieves the specified chain from the context."] pub fn get_chain (& self , index : usize) -> Option < CertChain > { let cert_chain = unsafe { let cert_chain = * self . 0 ; if index >= cert_chain . cChain as usize { None } else { let chain_slice = slice :: from_raw_parts (cert_chain . rgpChain , cert_chain . cChain as usize) ; Some (CertChain (chain_slice [index] , self . clone ())) } } ; cert_chain } # [doc = " Return an iterator over all certificate chains in this context"] pub fn chains (& self) -> CertificateChains { CertificateChains { context : self , idx : 0 , } } }
};
}
