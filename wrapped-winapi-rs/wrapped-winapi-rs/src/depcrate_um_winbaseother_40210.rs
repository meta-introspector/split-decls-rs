// Generated macro for other_40210 (other)
macro_rules! Depcrate_um_winbaseother_40210 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40210"}
// Dependencies: {}
extern "system" { pub fn RegisterApplicationRecoveryCallback (pRecoveyCallback : APPLICATION_RECOVERY_CALLBACK , pvParameter : PVOID , dwPingInterval : DWORD , dwFlags : DWORD ,) -> HRESULT ; pub fn UnregisterApplicationRecoveryCallback () -> HRESULT ; pub fn RegisterApplicationRestart (pwzCommandline : PCWSTR , dwFlags : DWORD ,) -> HRESULT ; pub fn UnregisterApplicationRestart () -> HRESULT ; pub fn GetApplicationRecoveryCallback (hProcess : HANDLE , pRecoveryCallback : * mut APPLICATION_RECOVERY_CALLBACK , ppvParameter : * mut PVOID , pdwPingInterval : PDWORD , pdwFlags : PDWORD ,) -> HRESULT ; pub fn GetApplicationRestartSettings (hProcess : HANDLE , pwzCommandline : PWSTR , pcchSize : PDWORD , pdwFlags : PDWORD ,) -> HRESULT ; pub fn ApplicationRecoveryInProgress (pbCancelled : PBOOL ,) -> HRESULT ; pub fn ApplicationRecoveryFinished (bSuccess : BOOL ,) ; }
};
}
