// Generated macro for impl_825 (impl)
macro_rules! Depcrate_crypto_ring_signimpl_825 {
() => {
// Module: crate::crypto::ring::sign
// Provides: {"impl_825"}
// Dependencies: {}
impl EcdsaSigner { # [doc = " Make a new [`EcdsaSigner`] from a DER encoding in PKCS#8 or SEC1"] # [doc = " format, expecting a key usable with precisely the given signature"] # [doc = " scheme."] fn new (der : & PrivateKeyDer < '_ > , scheme : SignatureScheme , sigalg : & 'static signature :: EcdsaSigningAlgorithm ,) -> Result < Self , () > { let rng = SystemRandom :: new () ; let key_pair = match der { PrivateKeyDer :: Sec1 (sec1) => { Self :: convert_sec1_to_pkcs8 (scheme , sigalg , sec1 . secret_sec1_der () , & rng) ? } PrivateKeyDer :: Pkcs8 (pkcs8) => { EcdsaKeyPair :: from_pkcs8 (sigalg , pkcs8 . secret_pkcs8_der () , & rng) . map_err (| _ | ()) ? } _ => return Err (()) , } ; Ok (Self { key : Arc :: new (key_pair) , scheme , }) } # [doc = " Convert a SEC1 encoding to PKCS8, and ask ring to parse it.  This"] # [doc = " can be removed once <https://github.com/briansmith/ring/pull/1456>"] # [doc = " (or equivalent) is landed."] fn convert_sec1_to_pkcs8 (scheme : SignatureScheme , sigalg : & 'static signature :: EcdsaSigningAlgorithm , maybe_sec1_der : & [u8] , rng : & dyn SecureRandom ,) -> Result < EcdsaKeyPair , () > { let pkcs8_prefix = match scheme { SignatureScheme :: ECDSA_NISTP256_SHA256 => & Self :: PKCS8_PREFIX_ECDSA_NISTP256 , SignatureScheme :: ECDSA_NISTP384_SHA384 => & Self :: PKCS8_PREFIX_ECDSA_NISTP384 , _ => unreachable ! () , } ; let sec1_wrap = wrap_in_octet_string (maybe_sec1_der) ; let pkcs8 = wrap_concat_in_sequence (pkcs8_prefix , & sec1_wrap) ; EcdsaKeyPair :: from_pkcs8 (sigalg , & pkcs8 , rng) . map_err (| _ | ()) } fn sign (& self , message : & [u8]) -> Result < Vec < u8 > , Error > { let rng = SystemRandom :: new () ; self . key . sign (& rng , message) . map_err (| _ | Error :: General ("signing failed" . into ())) . map (| sig | sig . as_ref () . into ()) } const PKCS8_PREFIX_ECDSA_NISTP256 : & [u8] = b"\x02\x01\x00\
      \x30\x13\
      \x06\x07\x2a\x86\x48\xce\x3d\x02\x01\
      \x06\x08\x2a\x86\x48\xce\x3d\x03\x01\x07" ; const PKCS8_PREFIX_ECDSA_NISTP384 : & [u8] = b"\x02\x01\x00\
     \x30\x10\
     \x06\x07\x2a\x86\x48\xce\x3d\x02\x01\
     \x06\x05\x2b\x81\x04\x00\x22" ; }
};
}
