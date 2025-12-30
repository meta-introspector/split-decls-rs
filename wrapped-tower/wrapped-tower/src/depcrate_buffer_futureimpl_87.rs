// Generated macro for impl_87 (impl)
macro_rules! Depcrate_buffer_futureimpl_87 {
() => {
// Module: crate::buffer::future
// Provides: {"impl_87"}
// Dependencies: {}
impl < F , T , E > Future for ResponseFuture < F > where F : Future < Output = Result < T , E > > , E : Into < crate :: BoxError > , { type Output = Result < T , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state . as_mut () . project () { ResponseStateProj :: Failed { error } => { return Poll :: Ready (Err (error . take () . expect ("polled after error"))) ; } ResponseStateProj :: Rx { rx } => match ready ! (rx . poll (cx)) { Ok (Ok (fut)) => this . state . set (ResponseState :: Poll { fut }) , Ok (Err (e)) => return Poll :: Ready (Err (e . into ())) , Err (_) => return Poll :: Ready (Err (Closed :: new () . into ())) , } , ResponseStateProj :: Poll { fut } => return fut . poll (cx) . map_err (Into :: into) , } } } }
};
}
