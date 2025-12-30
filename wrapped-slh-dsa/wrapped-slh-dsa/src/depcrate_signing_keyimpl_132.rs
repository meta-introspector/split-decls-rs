// Generated macro for impl_132 (impl)
macro_rules! Depcrate_signing_keyimpl_132 {
() => {
// Module: crate::signing_key
// Provides: {"impl_132"}
// Dependencies: {}
impl < P : ParameterSet > RandomizedSigner < Signature < P > > for SigningKey < P > { fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8] ,) -> Result < Signature < P > , signature :: Error > { self . try_multipart_sign_with_rng (rng , & [msg]) } }
};
}
