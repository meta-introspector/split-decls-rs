// Generated macro for IOCTL_SMARTCARD_EJECT (const)
macro_rules! Depcrate_um_winsmcrdIOCTL_SMARTCARD_EJECT {
() => {
// Module: crate::um::winsmcrd
// Provides: {"IOCTL_SMARTCARD_EJECT"}
// Dependencies: {}
pub const IOCTL_SMARTCARD_EJECT : DWORD = CTL_CODE ! (FILE_DEVICE_SMARTCARD , 6 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
