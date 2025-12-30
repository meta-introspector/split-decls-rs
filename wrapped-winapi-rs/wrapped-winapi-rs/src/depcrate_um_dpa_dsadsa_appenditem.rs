// Generated macro for DSA_AppendItem (function)
macro_rules! Depcrate_um_dpa_dsaDSA_AppendItem {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"DSA_AppendItem"}
// Dependencies: {}
# [inline] pub unsafe fn DSA_AppendItem (hdsa : HDSA , pitem : * const c_void) -> c_int { DSA_InsertItem (hdsa , DA_LAST , pitem) }
};
}
