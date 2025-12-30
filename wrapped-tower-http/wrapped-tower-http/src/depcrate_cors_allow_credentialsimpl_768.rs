// Generated macro for impl_768 (impl)
macro_rules! Depcrate_cors_allow_credentialsimpl_768 {
() => {
// Module: crate::cors::allow_credentials
// Provides: {"impl_768"}
// Dependencies: {}
impl From < bool > for AllowCredentials { fn from (v : bool) -> Self { match v { true => Self (AllowCredentialsInner :: Yes) , false => Self (AllowCredentialsInner :: No) , } } }
};
}
