// Generated macro for impl_674 (impl)
macro_rules! Depcrate_follow_redirectimpl_674 {
() => {
// Module: crate::follow_redirect
// Provides: {"impl_674"}
// Dependencies: {}
impl < ReqBody , ResBody , S , P > Service < Request < ReqBody > > for FollowRedirect < S , P > where S : Service < Request < ReqBody > , Response = Response < ResBody > > + Clone , ReqBody : Body + Default , P : Policy < ReqBody , S :: Error > + Clone , { type Response = Response < ResBody > ; type Error = S :: Error ; type Future = ResponseFuture < S , ReqBody , P > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { let service = self . inner . clone () ; let mut service = mem :: replace (& mut self . inner , service) ; let mut policy = self . policy . clone () ; let mut body = BodyRepr :: None ; body . try_clone_from (req . body () , & policy) ; policy . on_request (& mut req) ; ResponseFuture { method : req . method () . clone () , uri : req . uri () . clone () , version : req . version () , headers : req . headers () . clone () , body , future : Either :: Left (service . call (req)) , service , policy , } } }
};
}
