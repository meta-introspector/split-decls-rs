// Generated macro for other_37856 (other)
macro_rules! Depcrate_um_setupapiother_37856 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37856"}
// Dependencies: {}
extern "system" { pub fn SetupInstallServicesFromInfSectionA (InfHandle : HINF , SectionName : PCSTR , Flags : DWORD ,) -> BOOL ; pub fn SetupInstallServicesFromInfSectionW (InfHandle : HINF , SectionName : PCWSTR , Flags : DWORD ,) -> BOOL ; pub fn SetupInstallServicesFromInfSectionExA (InfHandle : HINF , SectionName : PCSTR , Flags : DWORD , DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA , Reserved1 : PVOID , Reserved2 : PVOID ,) -> BOOL ; pub fn SetupInstallServicesFromInfSectionExW (InfHandle : HINF , SectionName : PCWSTR , Flags : DWORD , DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA , Reserved1 : PVOID , Reserved2 : PVOID ,) -> BOOL ; pub fn InstallHinfSectionA (Window : HWND , ModuleHandle : HINSTANCE , CommandLine : PCSTR , ShowCommand : INT ,) -> () ; pub fn InstallHinfSectionW (Window : HWND , ModuleHandle : HINSTANCE , CommandLine : PCWSTR , ShowCommand : INT ,) -> () ; }
};
}
