// Generated macro for DecodePublicKey (trait)
macro_rules! Depcrate_traitsDecodePublicKey {
() => {
// Module: crate::traits
// Provides: {"DecodePublicKey"}
// Dependencies: {}
# [doc = " Parse a public key object from an encoded SPKI document."] pub trait DecodePublicKey : Sized { # [doc = " Deserialize object from ASN.1 DER-encoded [`SubjectPublicKeyInfo`]"] # [doc = " (binary format)."] fn from_public_key_der (bytes : & [u8]) -> Result < Self > ; # [doc = " Deserialize PEM-encoded [`SubjectPublicKeyInfo`]."] # [doc = ""] # [doc = " Keys in this format begin with the following delimiter:"] # [doc = ""] # [doc = " ```text"] # [doc = " -----BEGIN PUBLIC KEY-----"] # [doc = " ```"] # [cfg (feature = "pem")] fn from_public_key_pem (s : & str) -> Result < Self > { let (label , doc) = Document :: from_pem (s) ? ; SubjectPublicKeyInfoRef :: validate_pem_label (label) ? ; Self :: from_public_key_der (doc . as_bytes ()) } # [doc = " Load public key object from an ASN.1 DER-encoded file on the local"] # [doc = " filesystem (binary format)."] # [cfg (feature = "std")] fn read_public_key_der_file (path : impl AsRef < Path >) -> Result < Self > { let doc = Document :: read_der_file (path) ? ; Self :: from_public_key_der (doc . as_bytes ()) } # [doc = " Load public key object from a PEM-encoded file on the local filesystem."] # [cfg (all (feature = "pem" , feature = "std"))] fn read_public_key_pem_file (path : impl AsRef < Path >) -> Result < Self > { let (label , doc) = Document :: read_pem_file (path) ? ; SubjectPublicKeyInfoRef :: validate_pem_label (& label) ? ; Self :: from_public_key_der (doc . as_bytes ()) } }
};
}
