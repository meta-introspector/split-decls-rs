// Generated macro for other_37900 (other)
macro_rules! Depcrate_um_setupapiother_37900 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37900"}
// Dependencies: {}
extern "system" { pub fn SetupDiOpenClassRegKeyExA (ClassGuid : * const GUID , samDesired : REGSAM , Flags : DWORD , MachineName : PCSTR , Reserved : PVOID ,) -> HKEY ; pub fn SetupDiOpenClassRegKeyExW (ClassGuid : * const GUID , samDesired : REGSAM , Flags : DWORD , MachineName : PCWSTR , Reserved : PVOID ,) -> HKEY ; pub fn SetupDiCreateDeviceInterfaceRegKeyA (DeviceInfoSet : HDEVINFO , DeviceInterfaceData : PSP_DEVICE_INTERFACE_DATA , Reserved : DWORD , samDesired : REGSAM , InfHandle : HINF , InfSectionName : PCSTR ,) -> HKEY ; pub fn SetupDiCreateDeviceInterfaceRegKeyW (DeviceInfoSet : HDEVINFO , DeviceInterfaceData : PSP_DEVICE_INTERFACE_DATA , Reserved : DWORD , samDesired : REGSAM , InfHandle : HINF , InfSectionName : PCWSTR ,) -> HKEY ; pub fn SetupDiOpenDeviceInterfaceRegKey (DeviceInfoSet : HDEVINFO , DeviceInterfaceData : PSP_DEVICE_INTERFACE_DATA , Reserved : DWORD , samDesired : REGSAM ,) -> HKEY ; pub fn SetupDiDeleteDeviceInterfaceRegKey (DeviceInfoSet : HDEVINFO , DeviceInterfaceData : PSP_DEVICE_INTERFACE_DATA , Reserved : DWORD ,) -> BOOL ; }
};
}
