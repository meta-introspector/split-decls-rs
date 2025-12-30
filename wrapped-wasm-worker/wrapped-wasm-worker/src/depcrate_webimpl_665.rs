// Generated macro for impl_665 (impl)
macro_rules! Depcrate_webimpl_665 {
() => {
// Module: crate::web
// Provides: {"impl_665"}
// Dependencies: {}
impl < F , T > Future for ScopeFuture < '_ , '_ , F , T > where F : Future < Output = T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . 0 . poll (cx) } }
};
}
