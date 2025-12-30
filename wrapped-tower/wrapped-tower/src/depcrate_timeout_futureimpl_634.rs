// Generated macro for impl_634 (impl)
macro_rules! Depcrate_timeout_futureimpl_634 {
() => {
// Module: crate::timeout::future
// Provides: {"impl_634"}
// Dependencies: {}
impl < F , T , E > Future for ResponseFuture < F > where F : Future < Output = Result < T , E > > , E : Into < crate :: BoxError > , { type Output = Result < T , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; match this . response . poll (cx) { Poll :: Ready (v) => return Poll :: Ready (v . map_err (Into :: into)) , Poll :: Pending => { } } match this . sleep . poll (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (_) => Poll :: Ready (Err (Elapsed (()) . into ())) , } } }
};
}
