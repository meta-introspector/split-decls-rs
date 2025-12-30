// Generated macro for loop_while_non_fatal_error (function)
macro_rules! Depcrate_verify_certloop_while_non_fatal_error {
() => {
// Module: crate::verify_cert
// Provides: {"loop_while_non_fatal_error"}
// Dependencies: {}
fn loop_while_non_fatal_error < 'a , V : IntoIterator + 'a > (default_error : Error , values : V , mut f : impl FnMut (V :: Item) -> Result < & 'a TrustAnchor < 'a > , ControlFlow < Error , Error > > ,) -> Result < & 'a TrustAnchor < 'a > , ControlFlow < Error , Error > > { let mut error = default_error ; for v in values { match f (v) { Ok (anchor) => return Ok (anchor) , res @ Err (ControlFlow :: Break (_)) => return res , Err (ControlFlow :: Continue (new_error)) => error = error . most_specific (new_error) , } } Err (error . into ()) }
};
}
