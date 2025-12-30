// Generated macro for macro_37350 (macro)
macro_rules! Depcrate_um_setupapimacro_37350 {
() => {
// Module: crate::um::setupapi
// Provides: {"macro_37350"}
// Dependencies: {}
STRUCT ! { # [cfg_attr (target_arch = "x86" , repr (packed))] struct SP_DEVINSTALL_PARAMS_W { cbSize : DWORD , Flags : DWORD , FlagsEx : DWORD , hwndParent : HWND , InstallMsgHandler : PSP_FILE_CALLBACK_W , InstallMsgHandlerContext : PVOID , FileQueue : HSPFILEQ , ClassInstallReserved : ULONG_PTR , Reserved : DWORD , DriverPath : [WCHAR ; MAX_PATH] , } }
};
}
