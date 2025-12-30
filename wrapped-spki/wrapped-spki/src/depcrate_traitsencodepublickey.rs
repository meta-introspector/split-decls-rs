// Generated macro for EncodePublicKey (trait)
macro_rules! Depcrate_traitsEncodePublicKey {
() => {
// Module: crate::traits
// Provides: {"EncodePublicKey"}
// Dependencies: {}
# [doc = " Serialize a public key object to a SPKI-encoded document."] # [cfg (feature = "alloc")] pub trait EncodePublicKey { # [doc = " Serialize a [`Document`] containing a SPKI-encoded public key."] fn to_public_key_der (& self) -> Result < Document > ; # [doc = " Serialize this public key as PEM-encoded SPKI with the given [`LineEnding`]."] # [cfg (feature = "pem")] fn to_public_key_pem (& self , line_ending : LineEnding) -> Result < String > { let doc = self . to_public_key_der () ? ; Ok (doc . to_pem (SubjectPublicKeyInfoRef :: PEM_LABEL , line_ending) ?) } # [doc = " Write ASN.1 DER-encoded public key to the given path"] # [cfg (feature = "std")] fn write_public_key_der_file (& self , path : impl AsRef < Path >) -> Result < () > { Ok (self . to_public_key_der () ? . write_der_file (path) ?) } # [doc = " Write ASN.1 PEM-encoded public key to the given path"] # [cfg (all (feature = "pem" , feature = "std"))] fn write_public_key_pem_file (& self , path : impl AsRef < Path > , line_ending : LineEnding ,) -> Result < () > { let doc = self . to_public_key_der () ? ; Ok (doc . write_pem_file (path , SubjectPublicKeyInfoRef :: PEM_LABEL , line_ending) ?) } }
};
}
