// Generated macro for PublicTag (struct)
macro_rules! Depcrate_crypto_hmacPublicTag {
() => {
// Module: crate::crypto::hmac
// Provides: {"PublicTag"}
// Dependencies: {}
# [doc = " A non-secret HMAC tag, stored as a value."] # [doc = ""] # [doc = " A value of this type is **not** zeroized on drop."] # [doc = ""] # [doc = " A tag is \"public\" if it is published on the wire, as opposed to"] # [doc = " being used as key material. For example, the `verify_data` field"] # [doc = " of TLS `Finished` messages are public (as they are published on"] # [doc = " the wire in TLS1.2, or sent encrypted under pre-authenticated"] # [doc = " secrets in TLS1.3)."] # [derive (Clone)] pub (crate) struct PublicTag { buf : [u8 ; Tag :: MAX_LEN] , used : usize , }
};
}
