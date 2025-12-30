// Generated macro for macro_30142 (macro)
macro_rules! Depcrate_um_lmdfsmacro_30142 {
() => {
// Module: crate::um::lmdfs
// Provides: {"macro_30142"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] IFDEF ! { STRUCT ! { struct DFS_INFO_4_32 { EntryPath : ULONG , Comment : ULONG , State : DWORD , Timeout : ULONG , Guid : GUID , NumberOfStorages : DWORD , Storage : ULONG , } } pub type PDFS_INFO_4_32 = * mut DFS_INFO_4_32 ; pub type LPDFS_INFO_4_32 = * mut DFS_INFO_4_32 ; }
};
}
