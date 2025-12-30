// Generated macro for set_port (function)
macro_rules! Depcrate_quirksset_port {
() => {
// Module: crate::quirks
// Provides: {"set_port"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-port>"] # [allow (clippy :: result_unit_err)] pub fn set_port (url : & mut Url , new_port : & str) -> Result < () , () > { let result ; { let scheme = url . scheme () ; if ! url . has_host () || url . host () == Some (Host :: Domain ("")) || scheme == "file" { return Err (()) ; } result = Parser :: parse_port (Input :: new_no_trim (new_port) , | | default_port (scheme) , Context :: Setter ,) } if let Ok ((new_port , _remaining)) = result { url . set_port_internal (new_port) ; Ok (()) } else { Err (()) } }
};
}
