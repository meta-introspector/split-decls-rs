// Generated macro for set_host (function)
macro_rules! Depcrate_quirksset_host {
() => {
// Module: crate::quirks
// Provides: {"set_host"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-host>"] # [allow (clippy :: result_unit_err)] pub fn set_host (url : & mut Url , new_host : & str) -> Result < () , () > { if url . cannot_be_a_base () { return Err (()) ; } let input = Input :: new_no_trim (new_host) ; let host ; let opt_port ; { let scheme = url . scheme () ; let scheme_type = SchemeType :: from (scheme) ; if scheme_type == SchemeType :: File && new_host . is_empty () { url . set_host_internal (Host :: Domain ("" . into ()) , None) ; return Ok (()) ; } if let Ok ((h , remaining)) = Parser :: parse_host (input , scheme_type) { host = h ; opt_port = if let Some (remaining) = remaining . split_prefix (':') { if remaining . is_empty () { None } else { Parser :: parse_port (remaining , | | default_port (scheme) , Context :: Setter) . ok () . map (| (port , _remaining) | port) } } else { None } ; } else { return Err (()) ; } } if host == Host :: Domain ("" . to_string ()) && (! username (url) . is_empty () || matches ! (opt_port , Some (Some (_))) || url . port () . is_some ()) { return Err (()) ; } url . set_host_internal (host , opt_port) ; Ok (()) }
};
}
