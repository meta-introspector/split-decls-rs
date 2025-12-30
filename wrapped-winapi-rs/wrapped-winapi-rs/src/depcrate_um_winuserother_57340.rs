// Generated macro for other_57340 (other)
macro_rules! Depcrate_um_winuserother_57340 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57340"}
// Dependencies: {}
extern "system" { pub fn GetRawInputDeviceInfoA (hDevice : HANDLE , uiCommand : UINT , pData : LPVOID , pcbSize : PUINT ,) -> UINT ; pub fn GetRawInputDeviceInfoW (hDevice : HANDLE , uiCommand : UINT , pData : LPVOID , pcbSize : PUINT ,) -> UINT ; pub fn GetRawInputBuffer (pData : PRAWINPUT , pcbSize : PUINT , cbSizeHeader : UINT ,) -> UINT ; }
};
}
