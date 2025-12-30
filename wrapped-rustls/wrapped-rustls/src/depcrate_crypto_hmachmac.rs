// Generated macro for Hmac (trait)
macro_rules! Depcrate_crypto_hmacHmac {
() => {
// Module: crate::crypto::hmac
// Provides: {"Hmac"}
// Dependencies: {}
# [doc = " A concrete HMAC implementation, for a single cryptographic hash function."] # [doc = ""] # [doc = " You should have one object that implements this trait for HMAC-SHA256, another"] # [doc = " for HMAC-SHA384, etc."] pub trait Hmac : Send + Sync { # [doc = " Prepare to use `key` as a HMAC key."] fn with_key (& self , key : & [u8]) -> Box < dyn Key > ; # [doc = " Give the length of the underlying hash function.  In RFC2104 terminology this is `L`."] fn hash_output_len (& self) -> usize ; # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] fn fips (& self) -> bool { false } }
};
}
