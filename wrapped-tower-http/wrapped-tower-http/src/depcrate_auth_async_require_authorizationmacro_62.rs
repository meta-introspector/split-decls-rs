// Generated macro for macro_62 (macro)
macro_rules! Depcrate_auth_async_require_authorizationmacro_62 {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"macro_62"}
// Dependencies: {}
pin_project ! { # [project = StateProj] enum State < A , SFut > { Authorize { # [pin] authorize : A , } , Authorized { # [pin] fut : SFut , } , } }
};
}
