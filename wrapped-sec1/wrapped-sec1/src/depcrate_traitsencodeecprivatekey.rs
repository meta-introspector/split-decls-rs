// Generated macro for EncodeEcPrivateKey (trait)
macro_rules! Depcrate_traitsEncodeEcPrivateKey {
() => {
// Module: crate::traits
// Provides: {"EncodeEcPrivateKey"}
// Dependencies: {}
# [doc = " Serialize a [`EcPrivateKey`] to a SEC1 encoded document."] # [cfg (feature = "alloc")] pub trait EncodeEcPrivateKey { # [doc = " Serialize a [`SecretDocument`] containing a SEC1-encoded private key."] fn to_sec1_der (& self) -> Result < SecretDocument > ; # [doc = " Serialize this private key as PEM-encoded SEC1 with the given [`LineEnding`]."] # [doc = ""] # [doc = " To use the OS's native line endings, pass `Default::default()`."] # [cfg (feature = "pem")] fn to_sec1_pem (& self , line_ending : LineEnding) -> Result < Zeroizing < String > > { let doc = self . to_sec1_der () ? ; Ok (doc . to_pem (EcPrivateKey :: PEM_LABEL , line_ending) ?) } # [doc = " Write ASN.1 DER-encoded SEC1 private key to the given path."] # [cfg (feature = "std")] fn write_sec1_der_file (& self , path : impl AsRef < Path >) -> Result < () > { Ok (self . to_sec1_der () ? . write_der_file (path) ?) } # [doc = " Write ASN.1 PEM-encoded SEC1 private key to the given path."] # [cfg (all (feature = "pem" , feature = "std"))] fn write_sec1_pem_file (& self , path : impl AsRef < Path > , line_ending : LineEnding) -> Result < () > { let doc = self . to_sec1_der () ? ; Ok (doc . write_pem_file (path , EcPrivateKey :: PEM_LABEL , line_ending) ?) } }
};
}
