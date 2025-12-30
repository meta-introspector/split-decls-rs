// Generated macro for DPA_AppendPtr (function)
macro_rules! Depcrate_um_dpa_dsaDPA_AppendPtr {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"DPA_AppendPtr"}
// Dependencies: {}
# [inline] pub unsafe fn DPA_AppendPtr (hdpa : HDPA , pitem : * mut c_void) -> c_int { DPA_InsertPtr (hdpa , DA_LAST , pitem) }
};
}
