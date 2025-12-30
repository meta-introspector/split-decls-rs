// Generated macro for impl_63 (impl)
macro_rules! Depcrate_auth_async_require_authorizationimpl_63 {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"impl_63"}
// Dependencies: {}
impl < Auth , S , ReqBody , B > Future for ResponseFuture < Auth , S , ReqBody > where Auth : AsyncAuthorizeRequest < ReqBody , ResponseBody = B > , S : Service < Request < Auth :: RequestBody > , Response = Response < B > > , { type Output = Result < Response < B > , S :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state . as_mut () . project () { StateProj :: Authorize { authorize } => { let auth = ready ! (authorize . poll (cx)) ; match auth { Ok (req) => { let fut = this . service . call (req) ; this . state . set (State :: Authorized { fut }) } Err (res) => { return Poll :: Ready (Ok (res)) ; } } ; } StateProj :: Authorized { fut } => { return fut . poll (cx) ; } } } } }
};
}
