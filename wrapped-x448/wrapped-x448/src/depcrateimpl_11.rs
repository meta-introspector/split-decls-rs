// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl EphemeralSecret { fn new (value : Array < u8 , U56 >) -> Self { let mut out = Self (value) ; out . clamp () ; out } # [doc = " Generate a x448 `Secret` key."] pub fn random_from_rng < T > (csprng : & mut T) -> Self where T : RngCore + CryptoRng + ? Sized , { let mut bytes = Array :: default () ; csprng . fill_bytes (bytes . as_mut_slice ()) ; Self :: new (bytes) } # [doc = " Clamps the secret key according to RFC7748"] fn clamp (& mut self) { self . 0 [0] &= 252 ; self . 0 [55] |= 128 ; } # [doc = " Views an Secret as a Scalar"] fn as_scalar (& self) -> MontgomeryScalar { let secret = U448 :: from_le_slice (& self . 0) ; MontgomeryScalar :: from_uint_unchecked (secret) } # [doc = " Performs a Diffie-hellman key exchange between the secret key and an external public key"] pub fn diffie_hellman (& self , public_key : & PublicKey) -> SharedSecret { let shared_key = & public_key . 0 * & self . as_scalar () ; SharedSecret (shared_key) } # [doc = " Converts a secret into a byte array"] pub fn as_bytes (& self) -> & [u8 ; 56] { self . 0 . as_ref () } }
};
}
