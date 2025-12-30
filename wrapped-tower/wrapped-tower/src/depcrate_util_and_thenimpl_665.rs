// Generated macro for impl_665 (impl)
macro_rules! Depcrate_util_and_thenimpl_665 {
() => {
// Module: crate::util::and_then
// Provides: {"impl_665"}
// Dependencies: {}
impl < F1 , F2 : TryFuture , N > Future for AndThenFuture < F1 , F2 , N > where future :: AndThen < future :: ErrInto < F1 , F2 :: Error > , F2 , N > : Future , { type Output = < future :: AndThen < future :: ErrInto < F1 , F2 :: Error > , F2 , N > as Future > :: Output ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . inner . poll (cx) } }
};
}
