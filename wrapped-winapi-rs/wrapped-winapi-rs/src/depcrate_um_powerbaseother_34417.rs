// Generated macro for other_34417 (other)
macro_rules! Depcrate_um_powerbaseother_34417 {
() => {
// Module: crate::um::powerbase
// Provides: {"other_34417"}
// Dependencies: {}
extern "system" { pub fn CallNtPowerInformation (InformationLevel : POWER_INFORMATION_LEVEL , InputBuffer : PVOID , InputBufferLength : ULONG , OutputBuffer : PVOID , OutputBufferLength : ULONG ,) -> NTSTATUS ; pub fn GetPwrCapabilities (lpspc : PSYSTEM_POWER_CAPABILITIES ,) -> BOOLEAN ; pub fn PowerDeterminePlatformRoleEx (Version : ULONG ,) -> POWER_PLATFORM_ROLE ; pub fn PowerRegisterSuspendResumeNotification (Flags : DWORD , Recipient : HANDLE , RegistrationHandle : PHPOWERNOTIFY ,) -> DWORD ; pub fn PowerUnregisterSuspendResumeNotification (RegistrationHandle : HPOWERNOTIFY ,) -> DWORD ; }
};
}
