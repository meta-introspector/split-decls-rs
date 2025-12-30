// Generated macro for H3ConnectionError (enum)
macro_rules! Depcrate_http3_driverH3ConnectionError {
() => {
// Module: crate::http3::driver
// Provides: {"H3ConnectionError"}
// Dependencies: {}
# [doc = " The error type used internally in [H3Driver]."] # [doc = ""] # [doc = " Note that [`ApplicationOverQuic`] errors are not exposed to users at this"] # [doc = " time. The type is public to document the failure modes in [H3Driver]."] # [derive (Debug , PartialEq , Eq)] # [non_exhaustive] pub enum H3ConnectionError { # [doc = " The controller task was shut down and is no longer listening."] ControllerWentAway , # [doc = " Other error at the connection, but not stream level."] H3 (h3 :: Error) , # [doc = " Received a GOAWAY frame from the peer."] GoAway , # [doc = " Received data for a stream that was closed or never opened."] NonexistentStream , # [doc = " The server's post-accept timeout was hit."] # [doc = " The timeout can be configured in [`Http3Settings`]."] PostAcceptTimeout , }
};
}
