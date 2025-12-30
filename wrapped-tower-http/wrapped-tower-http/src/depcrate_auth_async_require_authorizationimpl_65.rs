// Generated macro for impl_65 (impl)
macro_rules! Depcrate_auth_async_require_authorizationimpl_65 {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"impl_65"}
// Dependencies: {}
impl < B , F , Fut , ReqBody , ResBody > AsyncAuthorizeRequest < B > for F where F : FnMut (Request < B >) -> Fut , Fut : Future < Output = Result < Request < ReqBody > , Response < ResBody > > > , { type RequestBody = ReqBody ; type ResponseBody = ResBody ; type Future = Fut ; fn authorize (& mut self , request : Request < B >) -> Self :: Future { self (request) } }
};
}
