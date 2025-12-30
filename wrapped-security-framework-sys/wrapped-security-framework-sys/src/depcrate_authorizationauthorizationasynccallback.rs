// Generated macro for AuthorizationAsyncCallback (type)
macro_rules! Depcrate_authorizationAuthorizationAsyncCallback {
() => {
// Module: crate::authorization
// Provides: {"AuthorizationAsyncCallback"}
// Dependencies: {}
pub type AuthorizationAsyncCallback = unsafe extern "C" fn (err : OSStatus , blockAuthorizedRights : * mut AuthorizationRights) ;
};
}
