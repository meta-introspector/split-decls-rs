// Generated macro for other_37877 (other)
macro_rules! Depcrate_um_setupapiother_37877 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37877"}
// Dependencies: {}
extern "system" { pub fn SetupDiCreateDeviceInfoA (DeviceInfoSet : HDEVINFO , DeviceName : PCSTR , ClassGuid : * const GUID , DeviceDescription : PCSTR , hwndParent : HWND , CreationFlags : DWORD , DeviceInfoData : PSP_DEVINFO_DATA ,) -> BOOL ; pub fn SetupDiCreateDeviceInfoW (DeviceInfoSet : HDEVINFO , DeviceName : PCWSTR , ClassGuid : * const GUID , DeviceDescription : PCWSTR , hwndParent : HWND , CreationFlags : DWORD , DeviceInfoData : PSP_DEVINFO_DATA ,) -> BOOL ; }
};
}
