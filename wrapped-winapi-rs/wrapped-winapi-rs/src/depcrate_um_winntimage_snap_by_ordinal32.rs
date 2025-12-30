// Generated macro for IMAGE_SNAP_BY_ORDINAL32 (function)
macro_rules! Depcrate_um_winntIMAGE_SNAP_BY_ORDINAL32 {
() => {
// Module: crate::um::winnt
// Provides: {"IMAGE_SNAP_BY_ORDINAL32"}
// Dependencies: {}
# [inline] pub fn IMAGE_SNAP_BY_ORDINAL32 (Ordinal : DWORD) -> bool { (Ordinal & IMAGE_ORDINAL_FLAG32) != 0 }
};
}
