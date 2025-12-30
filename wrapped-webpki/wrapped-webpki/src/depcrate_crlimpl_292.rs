// Generated macro for impl_292 (impl)
macro_rules! Depcrate_crlimpl_292 {
() => {
// Module: crate::crl
// Provides: {"impl_292"}
// Dependencies: {}
impl RevocationOptions < '_ > { # [allow (clippy :: too_many_arguments)] pub (crate) fn check (& self , path : & PathNode < '_ > , issuer_subject : untrusted :: Input < '_ > , issuer_spki : untrusted :: Input < '_ > , issuer_ku : Option < untrusted :: Input < '_ > > , supported_sig_algs : & [& dyn SignatureVerificationAlgorithm] , budget : & mut Budget , time : UnixTime ,) -> Result < Option < CertNotRevoked > , Error > { assert ! (public_values_eq (path . cert . issuer , issuer_subject)) ; if let (RevocationCheckDepth :: EndEntity , Role :: Issuer) = (self . depth , path . role ()) { return Ok (None) ; } let crl = self . crls . iter () . find (| candidate_crl | candidate_crl . authoritative (path)) ; use UnknownStatusPolicy :: * ; let crl = match (crl , self . status_policy) { (Some (crl) , _) => crl , (None , Allow) => return Ok (None) , (None , _) => return Err (Error :: UnknownRevocationStatus) , } ; crl . verify_signature (supported_sig_algs , issuer_spki , budget) . map_err (crl_signature_err) ? ; if self . expiration_policy == ExpirationPolicy :: Enforce { crl . check_expiration (time) ? ; } KeyUsageMode :: CrlSign . check (issuer_ku) ? ; let cert_serial = path . cert . serial . as_slice_less_safe () ; match crl . find_serial (cert_serial) ? { None => Ok (Some (CertNotRevoked :: assertion ())) , Some (_) => Err (Error :: CertRevoked) , } } }
};
}
