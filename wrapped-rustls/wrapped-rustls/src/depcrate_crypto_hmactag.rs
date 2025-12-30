// Generated macro for Tag (struct)
macro_rules! Depcrate_crypto_hmacTag {
() => {
// Module: crate::crypto::hmac
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " A secret HMAC tag, stored as a value."] # [doc = ""] # [doc = " The value is considered secret and sensitive, and is zeroized"] # [doc = " on drop."] # [doc = ""] # [doc = " This is suitable if the value is (for example) used as key"] # [doc = " material."] # [derive (Clone)] pub struct Tag (PublicTag) ;
};
}
