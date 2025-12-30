// Generated macro for impl_674 (impl)
macro_rules! Depcrate_webimpl_674 {
() => {
// Module: crate::web
// Provides: {"impl_674"}
// Dependencies: {}
impl < 'scope , 'env , F , T > Future for ScopeIntoJoinFuture < 'scope , 'env , F , T > where F : Future < Output = T > , { type Output = ScopeJoinFuture < 'scope , 'env , T > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . 0 . project () . 0 . poll_into_wait (cx) . map (ScopeFuture) . map (ScopeJoinFuture) } }
};
}
