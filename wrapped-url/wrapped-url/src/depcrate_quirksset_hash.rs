// Generated macro for set_hash (function)
macro_rules! Depcrate_quirksset_hash {
() => {
// Module: crate::quirks
// Provides: {"set_hash"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-hash>"] pub fn set_hash (url : & mut Url , new_hash : & str) { url . set_fragment (match new_hash { "" => None , _ if new_hash . starts_with ('#') => Some (& new_hash [1 ..]) , _ => Some (new_hash) , }) }
};
}
