// Generated macro for ServerErrorsFailureClass (enum)
macro_rules! Depcrate_classifyServerErrorsFailureClass {
() => {
// Module: crate::classify
// Provides: {"ServerErrorsFailureClass"}
// Dependencies: {}
# [doc = " The failure class for [`ServerErrorsAsFailures`]."] # [derive (Debug)] pub enum ServerErrorsFailureClass { # [doc = " A response was classified as a failure with the corresponding status."] StatusCode (StatusCode) , # [doc = " A response was classified as an error with the corresponding error description."] Error (String) , }
};
}
