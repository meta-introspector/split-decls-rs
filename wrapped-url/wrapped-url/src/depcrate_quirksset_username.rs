// Generated macro for set_username (function)
macro_rules! Depcrate_quirksset_username {
() => {
// Module: crate::quirks
// Provides: {"set_username"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-username>"] # [allow (clippy :: result_unit_err)] pub fn set_username (url : & mut Url , new_username : & str) -> Result < () , () > { url . set_username (new_username) }
};
}
