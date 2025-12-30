// Generated macro for impl_61 (impl)
macro_rules! Depcrate_filter_and_thenimpl_61 {
() => {
// Module: crate::filter::and_then
// Provides: {"impl_61"}
// Dependencies: {}
impl < T , F > Future for AndThenFuture < T , F > where T : Filter , F : Func < T :: Extract > , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : CombineRejection < T :: Error > , { type Output = Result < (< F :: Output as TryFuture > :: Ok ,) , < < F :: Output as TryFuture > :: Error as CombineRejection < T :: Error > > :: One , > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . state . poll (cx) } }
};
}
