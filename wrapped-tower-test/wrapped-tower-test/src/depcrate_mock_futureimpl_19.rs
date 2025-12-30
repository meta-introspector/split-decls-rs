// Generated macro for impl_19 (impl)
macro_rules! Depcrate_mock_futureimpl_19 {
() => {
// Module: crate::mock::future
// Provides: {"impl_19"}
// Dependencies: {}
impl < T > Future for ResponseFuture < T > { type Output = Result < T , Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { match self . project () . rx . as_pin_mut () { Some (rx) => match ready ! (rx . poll (cx)) { Ok (r) => Poll :: Ready (r) , Err (_) => Poll :: Ready (Err (error :: Closed :: new () . into ())) , } , None => Poll :: Ready (Err (error :: Closed :: new () . into ())) , } } }
};
}
