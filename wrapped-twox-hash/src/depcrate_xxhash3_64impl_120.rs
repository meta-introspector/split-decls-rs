// Generated macro for impl_120 (impl)
macro_rules! Depcrate_xxhash3_64impl_120 {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_120"}
// Dependencies: {}
impl < S > RawHasher < S > { # [doc = " Construct the hasher with the provided seed, secret, and"] # [doc = " temporary buffer."] pub fn new (secret_buffer : SecretBuffer < S >) -> Self { Self (RawHasherCore :: new (secret_buffer)) } # [doc = " Returns the secret."] pub fn into_secret (self) -> S { self . 0 . into_secret () } }
};
}
