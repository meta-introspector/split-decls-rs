// Generated macro for other_37963 (other)
macro_rules! Depcrate_um_setupapiother_37963 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37963"}
// Dependencies: {}
extern "system" { pub fn SetupDiGetClassDevPropertySheetsA (DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA , PropertySheetHeader : LPPROPSHEETHEADERA , PropertySheetHeaderPageListSize : DWORD , RequiredSize : PDWORD , PropertySheetType : DWORD ,) -> BOOL ; pub fn SetupDiGetClassDevPropertySheetsW (DeviceInfoSet : HDEVINFO , DeviceInfoData : PSP_DEVINFO_DATA , PropertySheetHeader : LPPROPSHEETHEADERW , PropertySheetHeaderPageListSize : DWORD , RequiredSize : PDWORD , PropertySheetType : DWORD ,) -> BOOL ; }
};
}
