// Generated macro for impl_60 (impl)
macro_rules! Depcrate_auth_async_require_authorizationimpl_60 {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"impl_60"}
// Dependencies: {}
impl < ReqBody , ResBody , S , Auth > Service < Request < ReqBody > > for AsyncRequireAuthorization < S , Auth > where Auth : AsyncAuthorizeRequest < ReqBody , ResponseBody = ResBody > , S : Service < Request < Auth :: RequestBody > , Response = Response < ResBody > > + Clone , { type Response = Response < ResBody > ; type Error = S :: Error ; type Future = ResponseFuture < Auth , S , ReqBody > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let mut inner = self . inner . clone () ; let authorize = self . auth . authorize (req) ; mem :: swap (& mut self . inner , & mut inner) ; ResponseFuture { state : State :: Authorize { authorize } , service : inner , } } }
};
}
