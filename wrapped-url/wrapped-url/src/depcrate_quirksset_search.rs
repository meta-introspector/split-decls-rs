// Generated macro for set_search (function)
macro_rules! Depcrate_quirksset_search {
() => {
// Module: crate::quirks
// Provides: {"set_search"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-search>"] pub fn set_search (url : & mut Url , new_search : & str) { url . set_query (match new_search { "" => None , _ if new_search . starts_with ('?') => Some (& new_search [1 ..]) , _ => Some (new_search) , }) }
};
}
