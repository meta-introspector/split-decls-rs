// Generated macro for impl_133 (impl)
macro_rules! Depcrate_signing_keyimpl_133 {
() => {
// Module: crate::signing_key
// Provides: {"impl_133"}
// Dependencies: {}
impl < P : ParameterSet > RandomizedMultipartSigner < Signature < P > > for SigningKey < P > { fn try_multipart_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [& [u8]] ,) -> Result < Signature < P > , Error > { let mut randomizer = Array :: < u8 , P :: N > :: default () ; rng . try_fill_bytes (randomizer . as_mut_slice ()) . map_err (| _ | signature :: Error :: new ()) ? ; self . raw_try_sign_with_context (msg , & [] , Some (& randomizer)) } }
};
}
