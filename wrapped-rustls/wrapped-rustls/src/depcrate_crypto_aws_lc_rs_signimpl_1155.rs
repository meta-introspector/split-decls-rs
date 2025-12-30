// Generated macro for impl_1155 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_signimpl_1155 {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"impl_1155"}
// Dependencies: {}
impl EcdsaSigner { # [doc = " Make a new [`EcdsaSigner`] from a DER encoding in PKCS#8 or SEC1"] # [doc = " format, expecting a key usable with precisely the given signature scheme."] fn new (der : & PrivateKeyDer < '_ > , scheme : SignatureScheme , sigalg : & 'static signature :: EcdsaSigningAlgorithm ,) -> Result < Self , () > { let key_pair = match der { PrivateKeyDer :: Sec1 (sec1) => { EcdsaKeyPair :: from_private_key_der (sigalg , sec1 . secret_sec1_der ()) . map_err (| _ | ()) ? } PrivateKeyDer :: Pkcs8 (pkcs8) => { EcdsaKeyPair :: from_pkcs8 (sigalg , pkcs8 . secret_pkcs8_der ()) . map_err (| _ | ()) ? } _ => return Err (()) , } ; Ok (Self { key : Arc :: new (key_pair) , scheme , }) } fn sign (& self , message : & [u8]) -> Result < Vec < u8 > , Error > { let rng = SystemRandom :: new () ; self . key . sign (& rng , message) . map_err (| _ | Error :: General ("signing failed" . into ())) . map (| sig | sig . as_ref () . into ()) } }
};
}
