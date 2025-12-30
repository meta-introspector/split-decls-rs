// Generated macro for impl_162 (impl)
macro_rules! Depcrate_filter_futureimpl_162 {
() => {
// Module: crate::filter::future
// Provides: {"impl_162"}
// Dependencies: {}
impl < P , S , Request > Future for AsyncResponseFuture < P , S , Request > where P : AsyncPredicate < Request > , S : Service < P :: Request > , S :: Error : Into < crate :: BoxError > , { type Output = Result < S :: Response , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state . as_mut () . project () { StateProj :: Check { mut check } => { let request = ready ! (check . as_mut () . poll (cx)) ? ; let response = this . service . call (request) ; this . state . set (State :: WaitResponse { response }) ; } StateProj :: WaitResponse { response } => { return response . poll (cx) . map_err (Into :: into) ; } } } } }
};
}
