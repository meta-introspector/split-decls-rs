// Generated macro for impl_50 (impl)
macro_rules! Depcrate_mockimpl_50 {
() => {
// Module: crate::mock
// Provides: {"impl_50"}
// Dependencies: {}
impl < T > SendResponse < T > { # [doc = " Resolve the pending request future for the linked request with the given response."] pub fn send_response (self , response : T) { let _ = self . tx . send (Ok (response)) ; } # [doc = " Resolve the pending request future for the linked request with the given error."] pub fn send_error < E : Into < Error > > (self , err : E) { let _ = self . tx . send (Err (err . into ())) ; } }
};
}
