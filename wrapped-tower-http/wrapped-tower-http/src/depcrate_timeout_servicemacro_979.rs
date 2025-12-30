// Generated macro for macro_979 (macro)
macro_rules! Depcrate_timeout_servicemacro_979 {
() => {
// Module: crate::timeout::service
// Provides: {"macro_979"}
// Dependencies: {}
pin_project ! { # [doc = " Response future for [`Timeout`]."] pub struct ResponseFuture < F > { # [pin] inner : F , # [pin] sleep : Sleep , status_code : StatusCode , } }
};
}
