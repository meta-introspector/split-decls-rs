// Generated macro for impl_1273 (impl)
macro_rules! Depcrate_validate_requestimpl_1273 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1273"}
// Dependencies: {}
impl < F , B > ResponseFuture < F , B > { fn future (future : F) -> Self { Self { kind : Kind :: Future { future } , } } fn invalid_header_value (res : Response < B >) -> Self { Self { kind : Kind :: Error { response : Some (res) , } , } } }
};
}
