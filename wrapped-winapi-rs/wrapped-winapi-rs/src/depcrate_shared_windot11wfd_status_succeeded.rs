// Generated macro for WFD_STATUS_SUCCEEDED (function)
macro_rules! Depcrate_shared_windot11WFD_STATUS_SUCCEEDED {
() => {
// Module: crate::shared::windot11
// Provides: {"WFD_STATUS_SUCCEEDED"}
// Dependencies: {}
# [inline] pub fn WFD_STATUS_SUCCEEDED (status : DOT11_WFD_STATUS_CODE) -> bool { status == DOT11_WFD_STATUS_SUCCESS || status == DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER }
};
}
