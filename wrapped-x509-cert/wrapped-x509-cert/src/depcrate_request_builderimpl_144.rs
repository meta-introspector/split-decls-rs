// Generated macro for impl_144 (impl)
macro_rules! Depcrate_request_builderimpl_144 {
() => {
// Module: crate::request::builder
// Provides: {"impl_144"}
// Dependencies: {}
impl < P > Builder for CertificateBuilder < P > where P : BuilderProfile , { type Output = Certificate ; fn finalize < S > (& mut self , cert_signer : & S) -> Result < vec :: Vec < u8 > > where S : Keypair + DynSignatureAlgorithmIdentifier , S :: VerifyingKey : EncodePublicKey , { let verifying_key = cert_signer . verifying_key () ; let signer_pub = SubjectPublicKeyInfo :: from_key (& verifying_key) ? ; self . tbs . signature = cert_signer . signature_algorithm_identifier () ? ; let mut default_extensions = self . profile . build_extensions (self . tbs . subject_public_key_info . owned_to_ref () , signer_pub . owned_to_ref () , & self . tbs ,) ? ; self . extensions . append (& mut default_extensions) ; if ! self . extensions . is_empty () { self . tbs . extensions = Some (self . extensions . clone ()) ; } if self . tbs . extensions . is_none () { if self . tbs . issuer_unique_id . is_some () || self . tbs . subject_unique_id . is_some () { self . tbs . version = Version :: V2 ; } else { self . tbs . version = Version :: V1 ; } } self . tbs . to_der () . map_err (Error :: from) } fn assemble < S > (self , signature : BitString , _signer : & S) -> Result < Self :: Output > where S : Keypair + DynSignatureAlgorithmIdentifier , S :: VerifyingKey : EncodePublicKey , { let signature_algorithm = self . tbs . signature . clone () ; Ok (Certificate { tbs_certificate : self . tbs , signature_algorithm , signature , }) } }
};
}
