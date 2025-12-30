// Generated macro for DeepRejectCtxt (struct)
macro_rules! Depcrate_fast_rejectDeepRejectCtxt {
() => {
// Module: crate::fast_reject
// Provides: {"DeepRejectCtxt"}
// Dependencies: {}
# [doc = " Given generic arguments, could they be unified after"] # [doc = " replacing parameters with inference variables or placeholders."] # [doc = " This behavior is toggled using the const generics."] # [doc = ""] # [doc = " We use this to quickly reject impl/wc candidates without needing"] # [doc = " to instantiate generic arguments/having to enter a probe."] # [doc = ""] # [doc = " We also use this function during coherence. For coherence the"] # [doc = " impls only have to overlap for some value, so we treat parameters"] # [doc = " on both sides like inference variables."] # [derive (Debug , Clone , Copy)] pub struct DeepRejectCtxt < I : Interner , const INSTANTIATE_LHS_WITH_INFER : bool , const INSTANTIATE_RHS_WITH_INFER : bool , > { _interner : PhantomData < I > , }
};
}
