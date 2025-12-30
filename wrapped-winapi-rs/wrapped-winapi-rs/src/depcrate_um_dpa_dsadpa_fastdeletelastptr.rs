// Generated macro for DPA_FastDeleteLastPtr (function)
macro_rules! Depcrate_um_dpa_dsaDPA_FastDeleteLastPtr {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"DPA_FastDeleteLastPtr"}
// Dependencies: {}
# [inline] pub unsafe fn DPA_FastDeleteLastPtr (hdpa : HDPA) -> c_int { * (hdpa as * mut c_int) -= 1 ; * (hdpa as * mut c_int) }
};
}
