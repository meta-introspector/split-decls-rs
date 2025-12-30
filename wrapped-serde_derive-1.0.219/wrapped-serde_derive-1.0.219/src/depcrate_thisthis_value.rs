// Generated macro for this_value (function)
macro_rules! Depcrate_thisthis_value {
() => {
// Module: crate::this
// Provides: {"this_value"}
// Dependencies: {}
pub fn this_value (cont : & Container) -> Path { if let Some (remote) = cont . attrs . remote () { let mut this = remote . clone () ; for segment in & mut this . segments { if let PathArguments :: AngleBracketed (arguments) = & mut segment . arguments { if arguments . colon2_token . is_none () { arguments . colon2_token = Some (Token ! [::] (arguments . lt_token . span)) ; } } } this } else { Path :: from (cont . ident . clone ()) } }
};
}
