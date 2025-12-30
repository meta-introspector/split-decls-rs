// Generated macro for impl_274 (impl)
macro_rules! Depcrate_limit_concurrency_futureimpl_274 {
() => {
// Module: crate::limit::concurrency::future
// Provides: {"impl_274"}
// Dependencies: {}
impl < F , T , E > Future for ResponseFuture < F > where F : Future < Output = Result < T , E > > , { type Output = Result < T , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (ready ! (self . project () . inner . poll (cx))) } }
};
}
