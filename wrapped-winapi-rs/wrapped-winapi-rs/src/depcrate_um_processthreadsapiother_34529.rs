// Generated macro for other_34529 (other)
macro_rules! Depcrate_um_processthreadsapiother_34529 {
() => {
// Module: crate::um::processthreadsapi
// Provides: {"other_34529"}
// Dependencies: {}
extern "system" { pub fn SetProcessInformation (hProcess : HANDLE , ProcessInformationClass : PROCESS_INFORMATION_CLASS , ProcessInformation : LPVOID , ProcessInformationSize : DWORD ,) -> BOOL ; pub fn GetProcessInformation (hProcess : HANDLE , ProcessInformationClass : PROCESS_INFORMATION_CLASS , ProcessInformation : LPVOID , ProcessInformationSize : DWORD ,) -> BOOL ; pub fn GetProcessShutdownParameters (lpdwLevel : LPDWORD , lpdwFlags : LPDWORD ,) -> BOOL ; }
};
}
