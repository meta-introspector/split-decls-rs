// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_classify_status_in_range_is_errorimpl_1051 {
() => {
// Module: crate::classify::status_in_range_is_error
// Provides: {"impl_1051"}
// Dependencies: {}
impl ClassifyResponse for StatusInRangeAsFailures { type FailureClass = StatusInRangeFailureClass ; type ClassifyEos = NeverClassifyEos < Self :: FailureClass > ; fn classify_response < B > (self , res : & http :: Response < B > ,) -> ClassifiedResponse < Self :: FailureClass , Self :: ClassifyEos > { if self . range . contains (& res . status () . as_u16 ()) { let class = StatusInRangeFailureClass :: StatusCode (res . status ()) ; ClassifiedResponse :: Ready (Err (class)) } else { ClassifiedResponse :: Ready (Ok (())) } } fn classify_error < E > (self , error : & E) -> Self :: FailureClass where E : std :: fmt :: Display + 'static , { StatusInRangeFailureClass :: Error (error . to_string ()) } }
};
}
