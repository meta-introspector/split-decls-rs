// Generated macro for other_57361 (other)
macro_rules! Depcrate_um_winuserother_57361 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57361"}
// Dependencies: {}
extern "system" { pub fn GetRawInputDeviceList (pRawInputDeviceList : PRAWINPUTDEVICELIST , puiNumDevices : PUINT , cbSize : UINT ,) -> UINT ; pub fn DefRawInputProc (paRawInput : * mut PRAWINPUT , nInput : INT , cbSizeHeader : UINT ,) -> LRESULT ; pub fn ChangeWindowMessageFilter (message : UINT , dwFlag : DWORD ,) -> BOOL ; }
};
}
