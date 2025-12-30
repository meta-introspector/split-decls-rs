// Generated macro for IOCTL_SMARTCARD_IS_PRESENT (const)
macro_rules! Depcrate_um_winsmcrdIOCTL_SMARTCARD_IS_PRESENT {
() => {
// Module: crate::um::winsmcrd
// Provides: {"IOCTL_SMARTCARD_IS_PRESENT"}
// Dependencies: {}
pub const IOCTL_SMARTCARD_IS_PRESENT : DWORD = CTL_CODE ! (FILE_DEVICE_SMARTCARD , 10 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
