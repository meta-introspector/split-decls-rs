// Generated macro for macro_37348 (macro)
macro_rules! Depcrate_um_setupapimacro_37348 {
() => {
// Module: crate::um::setupapi
// Provides: {"macro_37348"}
// Dependencies: {}
STRUCT ! { # [cfg_attr (target_arch = "x86" , repr (packed))] struct SP_DEVINSTALL_PARAMS_A { cbSize : DWORD , Flags : DWORD , FlagsEx : DWORD , hwndParent : HWND , InstallMsgHandler : PSP_FILE_CALLBACK_A , InstallMsgHandlerContext : PVOID , FileQueue : HSPFILEQ , ClassInstallReserved : ULONG_PTR , Reserved : DWORD , DriverPath : [CHAR ; MAX_PATH] , } }
};
}
