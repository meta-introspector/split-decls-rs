// Generated macro for DPA_GetPtrCount (function)
macro_rules! Depcrate_um_dpa_dsaDPA_GetPtrCount {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"DPA_GetPtrCount"}
// Dependencies: {}
# [inline] pub unsafe fn DPA_GetPtrCount (hdpa : HDPA) -> c_int { * (hdpa as * mut c_int) }
};
}
