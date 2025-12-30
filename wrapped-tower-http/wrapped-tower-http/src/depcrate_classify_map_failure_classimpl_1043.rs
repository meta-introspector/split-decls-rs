// Generated macro for impl_1043 (impl)
macro_rules! Depcrate_classify_map_failure_classimpl_1043 {
() => {
// Module: crate::classify::map_failure_class
// Provides: {"impl_1043"}
// Dependencies: {}
impl < C , F , NewClass > ClassifyResponse for MapFailureClass < C , F > where C : ClassifyResponse , F : FnOnce (C :: FailureClass) -> NewClass , { type FailureClass = NewClass ; type ClassifyEos = MapFailureClass < C :: ClassifyEos , F > ; fn classify_response < B > (self , res : & Response < B > ,) -> ClassifiedResponse < Self :: FailureClass , Self :: ClassifyEos > { match self . inner . classify_response (res) { ClassifiedResponse :: Ready (result) => ClassifiedResponse :: Ready (result . map_err (self . f)) , ClassifiedResponse :: RequiresEos (classify_eos) => { let mapped_classify_eos = MapFailureClass :: new (classify_eos , self . f) ; ClassifiedResponse :: RequiresEos (mapped_classify_eos) } } } fn classify_error < E > (self , error : & E) -> Self :: FailureClass where E : std :: fmt :: Display + 'static , { (self . f) (self . inner . classify_error (error)) } }
};
}
