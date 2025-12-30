// Generated macro for EFS_IS_APPX_VERSION (function)
macro_rules! Depcrate_um_winefsEFS_IS_APPX_VERSION {
() => {
// Module: crate::um::winefs
// Provides: {"EFS_IS_APPX_VERSION"}
// Dependencies: {}
# [inline] pub fn EFS_IS_APPX_VERSION (v : DWORD , subV : DWORD) -> bool { v == EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR && subV == EFS_PFILE_SUBVER_APPX }
};
}
