// Generated macro for impl_182 (impl)
macro_rules! Depcrate_filter_thenimpl_182 {
() => {
// Module: crate::filter::then
// Provides: {"impl_182"}
// Dependencies: {}
impl < T , F > Future for ThenFuture < T , F > where T : Filter , F : Func < T :: Extract > , F :: Output : Future + Send , { type Output = Result < (< F :: Output as Future > :: Output ,) , T :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . state . poll (cx) } }
};
}
