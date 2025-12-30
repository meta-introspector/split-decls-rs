// Generated macro for DPA_SetPtrCount (function)
macro_rules! Depcrate_um_dpa_dsaDPA_SetPtrCount {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"DPA_SetPtrCount"}
// Dependencies: {}
# [inline] pub unsafe fn DPA_SetPtrCount (hdpa : HDPA , cItems : c_int) { * (hdpa as * mut c_int) = cItems ; }
};
}
