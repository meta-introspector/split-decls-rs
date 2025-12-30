// Generated macro for impl_43 (impl)
macro_rules! Depcrate_auth_add_authorizationimpl_43 {
() => {
// Module: crate::auth::add_authorization
// Provides: {"impl_43"}
// Dependencies: {}
impl < S > Layer < S > for AddAuthorizationLayer { type Service = AddAuthorization < S > ; fn layer (& self , inner : S) -> Self :: Service { AddAuthorization { inner , value : self . value . clone () , } } }
};
}
