// Generated macro for impl_17 (impl)
macro_rules! Depcrate_errorimpl_17 {
() => {
// Module: crate::error
// Provides: {"impl_17"}
// Dependencies: {}
impl GraphQLBadRequest { # [doc = " Get the appropriate status code of the error."] # [must_use] pub fn status (& self) -> StatusCode { match self . 0 { ParseRequestError :: PayloadTooLarge => StatusCode :: PAYLOAD_TOO_LARGE , _ => StatusCode :: BAD_REQUEST , } } }
};
}
