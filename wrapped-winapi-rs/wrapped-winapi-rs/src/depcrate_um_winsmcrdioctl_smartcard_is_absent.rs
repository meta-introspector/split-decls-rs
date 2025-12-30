// Generated macro for IOCTL_SMARTCARD_IS_ABSENT (const)
macro_rules! Depcrate_um_winsmcrdIOCTL_SMARTCARD_IS_ABSENT {
() => {
// Module: crate::um::winsmcrd
// Provides: {"IOCTL_SMARTCARD_IS_ABSENT"}
// Dependencies: {}
pub const IOCTL_SMARTCARD_IS_ABSENT : DWORD = CTL_CODE ! (FILE_DEVICE_SMARTCARD , 11 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
