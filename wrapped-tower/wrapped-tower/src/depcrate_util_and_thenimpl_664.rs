// Generated macro for impl_664 (impl)
macro_rules! Depcrate_util_and_thenimpl_664 {
() => {
// Module: crate::util::and_then
// Provides: {"impl_664"}
// Dependencies: {}
impl < F1 , F2 : TryFuture , N > std :: fmt :: Debug for AndThenFuture < F1 , F2 , N > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("AndThenFuture") . field (& format_args ! ("...")) . finish () } }
};
}
