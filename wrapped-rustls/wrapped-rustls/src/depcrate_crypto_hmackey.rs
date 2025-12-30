// Generated macro for Key (trait)
macro_rules! Depcrate_crypto_hmacKey {
() => {
// Module: crate::crypto::hmac
// Provides: {"Key"}
// Dependencies: {}
# [doc = " A HMAC key that is ready for use."] # [doc = ""] # [doc = " The algorithm used is implicit in the `Hmac` object that produced the key."] pub trait Key : Send + Sync { # [doc = " Calculates a tag over `data` -- a slice of byte slices."] fn sign (& self , data : & [& [u8]]) -> Tag { self . sign_concat (& [] , data , & []) } # [doc = " Calculates a tag over the concatenation of `first`, the items in `middle`, and `last`."] fn sign_concat (& self , first : & [u8] , middle : & [& [u8]] , last : & [u8]) -> Tag ; # [doc = " Returns the length of the tag returned by a computation using"] # [doc = " this key."] fn tag_len (& self) -> usize ; }
};
}
