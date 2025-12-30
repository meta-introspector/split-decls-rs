// Generated macro for impl_129 (impl)
macro_rules! Depcrate_signing_keyimpl_129 {
() => {
// Module: crate::signing_key
// Provides: {"impl_129"}
// Dependencies: {}
impl < P : ParameterSet > TryFrom < & [u8] > for SigningKey < P > { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { if bytes . len () != P :: SkLen :: USIZE { return Err (Error :: new ()) ; } let (sk_seed_bytes , rest) = bytes . split_at (P :: N :: USIZE) ; let (sk_prf_bytes , verifying_key_bytes) = rest . split_at (P :: N :: USIZE) ; let verifying_key = VerifyingKey :: try_from (verifying_key_bytes) ? ; Ok (SigningKey { sk_seed : SkSeed :: from (sk_seed_bytes) , sk_prf : SkPrf :: from (sk_prf_bytes) , verifying_key , }) } }
};
}
