// Generated macro for StatusInRangeFailureClass (enum)
macro_rules! Depcrate_classify_status_in_range_is_errorStatusInRangeFailureClass {
() => {
// Module: crate::classify::status_in_range_is_error
// Provides: {"StatusInRangeFailureClass"}
// Dependencies: {}
# [doc = " The failure class for [`StatusInRangeAsFailures`]."] # [derive (Debug)] pub enum StatusInRangeFailureClass { # [doc = " A response was classified as a failure with the corresponding status."] StatusCode (StatusCode) , # [doc = " A response was classified as an error with the corresponding error description."] Error (String) , }
};
}
