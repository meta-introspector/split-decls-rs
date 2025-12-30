// Generated macro for stable_hash (function)
macro_rules! Depcrate_tidystable_hash {
() => {
// Module: crate::tidy
// Provides: {"stable_hash"}
// Dependencies: {}
# [allow (deprecated)] fn stable_hash (text : & str) -> u64 { use std :: hash :: { Hash , Hasher , SipHasher } ; let text = text . replace ('\r' , "") ; let mut hasher = SipHasher :: default () ; text . hash (& mut hasher) ; hasher . finish () }
};
}
