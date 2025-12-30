// Generated macro for impl_149 (impl)
macro_rules! Depcrate_request_builderimpl_149 {
() => {
// Module: crate::request::builder
// Provides: {"impl_149"}
// Dependencies: {}
impl < P > Builder for CrlBuilder < P > where P : certificate :: Profile , { type Output = CertificateList < P > ; fn finalize < S > (& mut self , cert_signer : & S) -> Result < vec :: Vec < u8 > > where S : Keypair + DynSignatureAlgorithmIdentifier , S :: VerifyingKey : EncodePublicKey , { self . tbs . signature = cert_signer . signature_algorithm_identifier () ? ; self . tbs . to_der () . map_err (Error :: from) } fn assemble < S > (self , signature : BitString , _signer : & S) -> Result < Self :: Output > where S : Keypair + DynSignatureAlgorithmIdentifier , S :: VerifyingKey : EncodePublicKey , { let signature_algorithm = self . tbs . signature . clone () ; Ok (CertificateList { tbs_cert_list : self . tbs , signature_algorithm , signature , }) } }
};
}
