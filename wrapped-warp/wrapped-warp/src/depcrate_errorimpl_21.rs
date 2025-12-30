// Generated macro for impl_21 (impl)
macro_rules! Depcrate_errorimpl_21 {
() => {
// Module: crate::error
// Provides: {"impl_21"}
// Dependencies: {}
impl Reply for GraphQLBadRequest { fn into_response (self) -> Response < Body > { Response :: builder () . status (self . status ()) . body (Body :: from (self . 0 . to_string ())) . unwrap () } }
};
}
