// Generated macro for IOCTL_CHANGER_GET_ELEMENT_STATUS (const)
macro_rules! Depcrate_um_winioctlIOCTL_CHANGER_GET_ELEMENT_STATUS {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_CHANGER_GET_ELEMENT_STATUS"}
// Dependencies: {}
pub const IOCTL_CHANGER_GET_ELEMENT_STATUS : DWORD = CTL_CODE ! (IOCTL_CHANGER_BASE , 0x0005 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
