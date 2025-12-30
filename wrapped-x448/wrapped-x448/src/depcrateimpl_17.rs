// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [cfg (feature = "static_secrets")] impl StaticSecret { fn new (value : Array < u8 , U56 >) -> Self { let mut out = Self (value) ; out . clamp () ; out } # [doc = " Generate a new [`StaticSecret`] with the supplied RNG."] pub fn random_from_rng < R : CryptoRng + ? Sized > (csprng : & mut R) -> Self { let mut bytes = Array :: default () ; csprng . fill_bytes (bytes . as_mut_slice ()) ; Self :: new (bytes) } # [doc = " Clamps the secret key according to RFC7748"] fn clamp (& mut self) { self . 0 [0] &= 252 ; self . 0 [55] |= 128 ; } # [doc = " Views an Secret as a Scalar"] fn as_scalar (& self) -> MontgomeryScalar { let secret = U448 :: from_le_slice (& self . 0) ; MontgomeryScalar :: from_uint_unchecked (secret) } # [doc = " Perform a Diffie-Hellman key agreement between `self` and"] # [doc = " `their_public` key to produce a `SharedSecret`."] pub fn diffie_hellman (& self , their_public : & PublicKey) -> SharedSecret { let shared_key = & their_public . 0 * & self . as_scalar () ; SharedSecret (shared_key) } # [doc = " View this key as a byte array."] # [inline] pub fn as_bytes (& self) -> & [u8 ; 56] { self . 0 . as_ref () } }
};
}
