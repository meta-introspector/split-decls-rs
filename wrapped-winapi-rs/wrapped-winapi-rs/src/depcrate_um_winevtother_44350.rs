// Generated macro for other_44350 (other)
macro_rules! Depcrate_um_winevtother_44350 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44350"}
// Dependencies: {}
extern "system" { pub fn EvtExportLog (Session : EVT_HANDLE , Path : LPCWSTR , Query : LPCWSTR , TargetFilePath : LPCWSTR , Flags : DWORD ,) -> BOOL ; pub fn EvtArchiveExportedLog (Session : EVT_HANDLE , LogFilePath : LPCWSTR , Locale : LCID , Flags : DWORD ,) -> BOOL ; }
};
}
