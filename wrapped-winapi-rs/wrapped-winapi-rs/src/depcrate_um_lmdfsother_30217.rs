// Generated macro for other_30217 (other)
macro_rules! Depcrate_um_lmdfsother_30217 {
() => {
// Module: crate::um::lmdfs
// Provides: {"other_30217"}
// Dependencies: {}
extern "system" { pub fn NetDfsRename (Path : LPWSTR , NewPath : LPWSTR ,) -> NET_API_STATUS ; pub fn NetDfsAddRootTarget (pDfsPath : LPWSTR , pTargetPath : LPWSTR , MajorVersion : ULONG , pComment : LPWSTR , Flags : ULONG ,) -> NET_API_STATUS ; }
};
}
