// Generated macro for impl_883 (impl)
macro_rules! Depcrate_util_oneshotimpl_883 {
() => {
// Module: crate::util::oneshot
// Provides: {"impl_883"}
// Dependencies: {}
impl < S : Service < Req > , Req > State < S , Req > { const fn not_ready (svc : S , req : Option < Req >) -> Self { Self :: NotReady { svc , req } } const fn called (fut : S :: Future) -> Self { Self :: Called { fut } } }
};
}
