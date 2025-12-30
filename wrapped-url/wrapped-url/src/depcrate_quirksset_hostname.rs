// Generated macro for set_hostname (function)
macro_rules! Depcrate_quirksset_hostname {
() => {
// Module: crate::quirks
// Provides: {"set_hostname"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-hostname>"] # [allow (clippy :: result_unit_err)] pub fn set_hostname (url : & mut Url , new_hostname : & str) -> Result < () , () > { if url . cannot_be_a_base () { return Err (()) ; } let input = Input :: new_no_trim (new_hostname) ; let scheme_type = SchemeType :: from (url . scheme ()) ; if scheme_type == SchemeType :: File && new_hostname . is_empty () { url . set_host_internal (Host :: Domain ("" . into ()) , None) ; return Ok (()) ; } if let Ok ((host , remaining)) = Parser :: parse_host (input , scheme_type) { if remaining . starts_with (':') { return Err (()) ; } ; if let Host :: Domain (h) = & host { if h . is_empty () { if SchemeType :: from (url . scheme ()) == SchemeType :: SpecialNotFile || ! port (url) . is_empty () || ! url . username () . is_empty () || ! url . password () . unwrap_or ("") . is_empty () { return Err (()) ; } } } url . set_host_internal (host , None) ; Ok (()) } else { Err (()) } }
};
}
