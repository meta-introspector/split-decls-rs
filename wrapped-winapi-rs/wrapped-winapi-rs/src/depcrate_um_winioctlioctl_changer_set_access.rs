// Generated macro for IOCTL_CHANGER_SET_ACCESS (const)
macro_rules! Depcrate_um_winioctlIOCTL_CHANGER_SET_ACCESS {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_CHANGER_SET_ACCESS"}
// Dependencies: {}
pub const IOCTL_CHANGER_SET_ACCESS : DWORD = CTL_CODE ! (IOCTL_CHANGER_BASE , 0x0004 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
