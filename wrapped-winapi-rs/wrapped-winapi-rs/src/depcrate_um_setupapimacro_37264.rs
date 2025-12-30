// Generated macro for macro_37264 (macro)
macro_rules! Depcrate_um_setupapimacro_37264 {
() => {
// Module: crate::um::setupapi
// Provides: {"macro_37264"}
// Dependencies: {}
STRUCT ! { # [cfg_attr (target_arch = "x86" , repr (packed))] struct FILE_IN_CABINET_INFO_W { NameInCabinet : PCWSTR , FileSize : DWORD , Win32Error : DWORD , DosDate : WORD , DosTime : WORD , DosAttribs : WORD , FullTargetName : [WCHAR ; MAX_PATH] , } }
};
}
