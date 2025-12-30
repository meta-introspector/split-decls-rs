// Generated macro for other_37884 (other)
macro_rules! Depcrate_um_setupapiother_37884 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37884"}
// Dependencies: {}
extern "system" { pub fn SetupDiRegisterDeviceInfo (DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA , Flags : DWORD , CompareProc : PSP_DETSIG_CMPPROC , CompareContext : PVOID , DupDeviceInfoData : PSP_DEVINFO_DATA ,) -> BOOL ; }
};
}
