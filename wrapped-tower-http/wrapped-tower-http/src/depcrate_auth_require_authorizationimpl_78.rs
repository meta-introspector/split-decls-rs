// Generated macro for impl_78 (impl)
macro_rules! Depcrate_auth_require_authorizationimpl_78 {
() => {
// Module: crate::auth::require_authorization
// Provides: {"impl_78"}
// Dependencies: {}
impl < ResBody > Bearer < ResBody > { fn new (token : & str) -> Self where ResBody : Default , { Self { header_value : format ! ("Bearer {}" , token) . parse () . expect ("token is not a valid header value") , _ty : PhantomData , } } }
};
}
