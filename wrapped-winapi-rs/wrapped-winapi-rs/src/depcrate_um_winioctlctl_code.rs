// Generated macro for CTL_CODE (function)
macro_rules! Depcrate_um_winioctlCTL_CODE {
() => {
// Module: crate::um::winioctl
// Provides: {"CTL_CODE"}
// Dependencies: {}
# [inline] pub fn CTL_CODE (DeviceType : DWORD , Function : DWORD , Method : DWORD , Access : DWORD ,) -> DWORD { (DeviceType << 16) | (Access << 14) | (Function << 2) | Method }
};
}
