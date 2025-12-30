// Generated macro for other_38937 (other)
macro_rules! Depcrate_um_sysinfoapiother_38937 {
() => {
// Module: crate::um::sysinfoapi
// Provides: {"other_38937"}
// Dependencies: {}
extern "system" { pub fn SetComputerNameEx2W (NameType : COMPUTER_NAME_FORMAT , Flags : DWORD , lpBuffer : LPCWSTR ,) -> BOOL ; pub fn SetSystemTimeAdjustment (dwTimeAdjustment : DWORD , bTimeAdjustmentDisabled : BOOL ,) -> BOOL ; pub fn InstallELAMCertificateInfo (ELAMFile : HANDLE ,) -> BOOL ; pub fn GetProcessorSystemCycleTime (Group : USHORT , Buffer : PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION , ReturnedLength : PDWORD ,) -> BOOL ; pub fn SetComputerNameA (lpComputerName : LPCSTR ,) -> BOOL ; pub fn SetComputerNameW (lpComputerName : LPCWSTR ,) -> BOOL ; pub fn SetComputerNameExA (NameType : COMPUTER_NAME_FORMAT , lpBuffer : LPCSTR ,) -> BOOL ; }
};
}
