// Generated macro for impl_46 (impl)
macro_rules! Depcrate_auth_add_authorizationimpl_46 {
() => {
// Module: crate::auth::add_authorization
// Provides: {"impl_46"}
// Dependencies: {}
impl < S , ReqBody , ResBody > Service < Request < ReqBody > > for AddAuthorization < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { req . headers_mut () . insert (http :: header :: AUTHORIZATION , self . value . clone ()) ; self . inner . call (req) } }
};
}
