// Generated macro for impl_56 (impl)
macro_rules! Depcrate_auth_async_require_authorizationimpl_56 {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"impl_56"}
// Dependencies: {}
impl < S , T > Layer < S > for AsyncRequireAuthorizationLayer < T > where T : Clone , { type Service = AsyncRequireAuthorization < S , T > ; fn layer (& self , inner : S) -> Self :: Service { AsyncRequireAuthorization :: new (inner , self . auth . clone ()) } }
};
}
