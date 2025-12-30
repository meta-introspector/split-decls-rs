// Generated macro for macro_2 (macro)
macro_rules! Depcrate_bindingsmacro_2 {
() => {
// Module: crate::bindings
// Provides: {"macro_2"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExW (lpservicename : PCWSTR , lphandlerproc : LPHANDLER_FUNCTION_EX , lpcontext : * const core :: ffi :: c_void) -> SERVICE_STATUS_HANDLE) ;
};
}
