// Generated macro for impl_83 (impl)
macro_rules! Depcrate_auth_require_authorizationimpl_83 {
() => {
// Module: crate::auth::require_authorization
// Provides: {"impl_83"}
// Dependencies: {}
impl < ResBody > Basic < ResBody > { fn new (username : & str , password : & str) -> Self where ResBody : Default , { let encoded = BASE64 . encode (format ! ("{}:{}" , username , password)) ; let header_value = format ! ("Basic {}" , encoded) . parse () . unwrap () ; Self { header_value , _ty : PhantomData , } } }
};
}
