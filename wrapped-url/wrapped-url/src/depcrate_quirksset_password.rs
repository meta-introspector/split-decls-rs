// Generated macro for set_password (function)
macro_rules! Depcrate_quirksset_password {
() => {
// Module: crate::quirks
// Provides: {"set_password"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-password>"] # [allow (clippy :: result_unit_err)] pub fn set_password (url : & mut Url , new_password : & str) -> Result < () , () > { url . set_password (if new_password . is_empty () { None } else { Some (new_password) }) }
};
}
