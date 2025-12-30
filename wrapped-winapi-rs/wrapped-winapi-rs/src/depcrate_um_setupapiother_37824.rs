// Generated macro for other_37824 (other)
macro_rules! Depcrate_um_setupapiother_37824 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37824"}
// Dependencies: {}
extern "system" { pub fn SetupInstallFromInfSectionA (Owner : HWND , InfHandle : HINF , SectionName : PCSTR , Flags : UINT , RelativeKeyRoot : HKEY , SourceRootPath : PCSTR , CopyFlags : UINT , MsgHandler : PSP_FILE_CALLBACK_A , Context : PVOID , DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA ,) -> BOOL ; pub fn SetupInstallFromInfSectionW (Owner : HWND , InfHandle : HINF , SectionName : PCWSTR , Flags : UINT , RelativeKeyRoot : HKEY , SourceRootPath : PCWSTR , CopyFlags : UINT , MsgHandler : PSP_FILE_CALLBACK_W , Context : PVOID , DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA ,) -> BOOL ; }
};
}
