// Generated macro for impl_142 (impl)
macro_rules! Depcrate_xxhash3_128impl_142 {
() => {
// Module: crate::xxhash3_128
// Provides: {"impl_142"}
// Dependencies: {}
impl < S > RawHasher < S > { # [doc = " Construct the hasher with the provided seed, secret, and"] # [doc = " temporary buffer."] pub fn new (secret_buffer : SecretBuffer < S >) -> Self { Self (RawHasherCore :: new (secret_buffer)) } # [doc = " Returns the secret."] pub fn into_secret (self) -> S { self . 0 . into_secret () } }
};
}
