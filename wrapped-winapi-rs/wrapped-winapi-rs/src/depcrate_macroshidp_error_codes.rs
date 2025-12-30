// Generated macro for HIDP_ERROR_CODES (macro)
macro_rules! Depcrate_macrosHIDP_ERROR_CODES {
() => {
// Module: crate::macros
// Provides: {"HIDP_ERROR_CODES"}
// Dependencies: {}
macro_rules ! HIDP_ERROR_CODES { ($ sev : expr , $ code : expr) => { ($ sev << 28) | (FACILITY_HID_ERROR_CODE << 16) | $ code } }
};
}
