// Generated macro for IOCTL_SMARTCARD_GET_LAST_ERROR (const)
macro_rules! Depcrate_um_winsmcrdIOCTL_SMARTCARD_GET_LAST_ERROR {
() => {
// Module: crate::um::winsmcrd
// Provides: {"IOCTL_SMARTCARD_GET_LAST_ERROR"}
// Dependencies: {}
pub const IOCTL_SMARTCARD_GET_LAST_ERROR : DWORD = CTL_CODE ! (FILE_DEVICE_SMARTCARD , 15 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
