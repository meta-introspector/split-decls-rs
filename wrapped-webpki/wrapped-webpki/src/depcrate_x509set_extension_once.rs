// Generated macro for set_extension_once (function)
macro_rules! Depcrate_x509set_extension_once {
() => {
// Module: crate::x509
// Provides: {"set_extension_once"}
// Dependencies: {}
pub (crate) fn set_extension_once < T > (destination : & mut Option < T > , parser : impl Fn () -> Result < T , Error > ,) -> Result < () , Error > { match destination { Some (..) => Err (Error :: ExtensionValueInvalid) , None => { * destination = Some (parser () ?) ; Ok (()) } } }
};
}
