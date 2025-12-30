// Generated macro for IOCTL_SMARTCARD_GET_STATE (const)
macro_rules! Depcrate_um_winsmcrdIOCTL_SMARTCARD_GET_STATE {
() => {
// Module: crate::um::winsmcrd
// Provides: {"IOCTL_SMARTCARD_GET_STATE"}
// Dependencies: {}
pub const IOCTL_SMARTCARD_GET_STATE : DWORD = CTL_CODE ! (FILE_DEVICE_SMARTCARD , 14 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
