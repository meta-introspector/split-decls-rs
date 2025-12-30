// Generated macro for impl_1068 (impl)
macro_rules! Depcrate_classifyimpl_1068 {
() => {
// Module: crate::classify
// Provides: {"impl_1068"}
// Dependencies: {}
impl ClassifyResponse for ServerErrorsAsFailures { type FailureClass = ServerErrorsFailureClass ; type ClassifyEos = NeverClassifyEos < ServerErrorsFailureClass > ; fn classify_response < B > (self , res : & Response < B > ,) -> ClassifiedResponse < Self :: FailureClass , Self :: ClassifyEos > { if res . status () . is_server_error () { ClassifiedResponse :: Ready (Err (ServerErrorsFailureClass :: StatusCode (res . status ()))) } else { ClassifiedResponse :: Ready (Ok (())) } } fn classify_error < E > (self , error : & E) -> Self :: FailureClass where E : fmt :: Display + 'static , { ServerErrorsFailureClass :: Error (error . to_string ()) } }
};
}
