// Generated macro for VALID_IMPERSONATION_LEVEL (function)
macro_rules! Depcrate_um_winntVALID_IMPERSONATION_LEVEL {
() => {
// Module: crate::um::winnt
// Provides: {"VALID_IMPERSONATION_LEVEL"}
// Dependencies: {}
# [inline] pub fn VALID_IMPERSONATION_LEVEL (L : SECURITY_IMPERSONATION_LEVEL) -> bool { (L >= SECURITY_MIN_IMPERSONATION_LEVEL) && (L <= SECURITY_MAX_IMPERSONATION_LEVEL) }
};
}
