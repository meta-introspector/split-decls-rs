// Generated macro for impl_712 (impl)
macro_rules! Depcrate_rejectimpl_712 {
() => {
// Module: crate::reject
// Provides: {"impl_712"}
// Dependencies: {}
impl IsReject for Rejection { fn status (& self) -> StatusCode { match self . reason { Reason :: NotFound => StatusCode :: NOT_FOUND , Reason :: Other (ref other) => other . status () , } } fn into_response (& self) -> crate :: reply :: Response { match self . reason { Reason :: NotFound => { let mut res = http :: Response :: default () ; * res . status_mut () = StatusCode :: NOT_FOUND ; res } Reason :: Other (ref other) => other . into_response () , } } }
};
}
