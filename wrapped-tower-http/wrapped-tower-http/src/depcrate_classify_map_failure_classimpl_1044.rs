// Generated macro for impl_1044 (impl)
macro_rules! Depcrate_classify_map_failure_classimpl_1044 {
() => {
// Module: crate::classify::map_failure_class
// Provides: {"impl_1044"}
// Dependencies: {}
impl < C , F , NewClass > ClassifyEos for MapFailureClass < C , F > where C : ClassifyEos , F : FnOnce (C :: FailureClass) -> NewClass , { type FailureClass = NewClass ; fn classify_eos (self , trailers : Option < & HeaderMap >) -> Result < () , Self :: FailureClass > { self . inner . classify_eos (trailers) . map_err (self . f) } fn classify_error < E > (self , error : & E) -> Self :: FailureClass where E : std :: fmt :: Display + 'static , { (self . f) (self . inner . classify_error (error)) } }
};
}
