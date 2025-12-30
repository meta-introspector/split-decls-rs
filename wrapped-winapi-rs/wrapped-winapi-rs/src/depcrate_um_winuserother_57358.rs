// Generated macro for other_57358 (other)
macro_rules! Depcrate_um_winuserother_57358 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57358"}
// Dependencies: {}
extern "system" { pub fn RegisterRawInputDevices (pRawInputDevices : PCRAWINPUTDEVICE , uiNumDevices : UINT , cbSize : UINT ,) -> BOOL ; pub fn GetRegisteredRawInputDevices (pRawInputDevices : PRAWINPUTDEVICE , puiNumDevices : PUINT , cbSize : UINT ,) -> UINT ; }
};
}
