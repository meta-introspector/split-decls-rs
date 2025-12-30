// Generated macro for impl_182 (impl)
macro_rules! Depcrate_tls_streamimpl_182 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_182"}
// Dependencies: {}
impl CertValidationResult { # [doc = " Returns the certificate that failed validation if applicable"] pub fn failed_certificate (& self) -> Option < CertContext > { if let Some (cert_chain) = self . chain . get_chain (self . chain_index as usize) { return cert_chain . get (self . element_index as usize) ; } None } # [doc = " Returns the final certificate chain in the certificate context if applicable"] pub fn chain (& self) -> Option < CertChain > { self . chain . final_chain () } # [doc = " Returns the result of the built-in certificate verification process."] pub fn result (& self) -> io :: Result < () > { if self . res as u32 != Foundation :: ERROR_SUCCESS { Err (io :: Error :: from_raw_os_error (self . res)) } else { Ok (()) } } }
};
}
