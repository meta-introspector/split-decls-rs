// Generated macro for set_protocol (function)
macro_rules! Depcrate_quirksset_protocol {
() => {
// Module: crate::quirks
// Provides: {"set_protocol"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-protocol>"] # [allow (clippy :: result_unit_err)] pub fn set_protocol (url : & mut Url , mut new_protocol : & str) -> Result < () , () > { if let Some (position) = new_protocol . find (':') { new_protocol = & new_protocol [.. position] ; } url . set_scheme (new_protocol) }
};
}
