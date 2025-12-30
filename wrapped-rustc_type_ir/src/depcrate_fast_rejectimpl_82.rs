// Generated macro for impl_82 (impl)
macro_rules! Depcrate_fast_rejectimpl_82 {
() => {
// Module: crate::fast_reject
// Provides: {"impl_82"}
// Dependencies: {}
impl < I : Interner > DeepRejectCtxt < I , false , true > { # [doc = " Treat parameters in the lhs as rigid, and in rhs as infer vars."] pub fn relate_rigid_infer (_interner : I) -> DeepRejectCtxt < I , false , true > { DeepRejectCtxt { _interner : PhantomData } } }
};
}
