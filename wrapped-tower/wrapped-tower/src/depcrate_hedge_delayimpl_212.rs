// Generated macro for impl_212 (impl)
macro_rules! Depcrate_hedge_delayimpl_212 {
() => {
// Module: crate::hedge::delay
// Provides: {"impl_212"}
// Dependencies: {}
impl < Request , S , T , E > Future for ResponseFuture < Request , S > where E : Into < crate :: BoxError > , S : Service < Request , Response = T , Error = E > , { type Output = Result < T , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state . as_mut () . project () { StateProj :: Delaying { delay , req } => { ready ! (delay . poll (cx)) ; let req = req . take () . expect ("Missing request in delay") ; let svc = this . service . take () . expect ("Missing service in delay") ; let fut = Oneshot :: new (svc , req) ; this . state . set (State :: called (fut)) ; } StateProj :: Called { fut } => { return fut . poll (cx) . map_err (Into :: into) ; } } ; } } }
};
}
