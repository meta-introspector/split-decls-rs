// Generated macro for SecureRandom (trait)
macro_rules! Depcrate_cryptoSecureRandom {
() => {
// Module: crate::crypto
// Provides: {"SecureRandom"}
// Dependencies: {}
# [doc = " A source of cryptographically secure randomness."] pub trait SecureRandom : Send + Sync + Debug { # [doc = " Fill the given buffer with random bytes."] # [doc = ""] # [doc = " The bytes must be sourced from a cryptographically secure random number"] # [doc = " generator seeded with good quality, secret entropy."] # [doc = ""] # [doc = " This is used for all randomness required by rustls, but not necessarily"] # [doc = " randomness required by the underlying cryptography library.  For example:"] # [doc = " [`SupportedKxGroup::start()`] requires random material to generate"] # [doc = " an ephemeral key exchange key, but this is not included in the interface with"] # [doc = " rustls: it is assumed that the cryptography library provides for this itself."] fn fill (& self , buf : & mut [u8]) -> Result < () , GetRandomFailed > ; # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] fn fips (& self) -> bool { false } }
};
}
