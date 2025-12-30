// Generated macro for impl_1064 (impl)
macro_rules! Depcrate_classifyimpl_1064 {
() => {
// Module: crate::classify
// Provides: {"impl_1064"}
// Dependencies: {}
impl < T > ClassifyEos for NeverClassifyEos < T > { type FailureClass = T ; fn classify_eos (self , _trailers : Option < & HeaderMap >) -> Result < () , Self :: FailureClass > { unreachable ! () } fn classify_error < E > (self , _error : & E) -> Self :: FailureClass where E : fmt :: Display + 'static , { unreachable ! () } }
};
}
