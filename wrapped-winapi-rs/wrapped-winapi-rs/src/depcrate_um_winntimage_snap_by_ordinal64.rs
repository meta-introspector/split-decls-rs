// Generated macro for IMAGE_SNAP_BY_ORDINAL64 (function)
macro_rules! Depcrate_um_winntIMAGE_SNAP_BY_ORDINAL64 {
() => {
// Module: crate::um::winnt
// Provides: {"IMAGE_SNAP_BY_ORDINAL64"}
// Dependencies: {}
# [inline] pub fn IMAGE_SNAP_BY_ORDINAL64 (Ordinal : ULONGLONG) -> bool { (Ordinal & IMAGE_ORDINAL_FLAG64) != 0 }
};
}
