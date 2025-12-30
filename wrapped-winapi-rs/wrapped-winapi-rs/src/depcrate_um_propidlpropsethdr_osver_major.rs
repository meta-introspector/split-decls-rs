// Generated macro for PROPSETHDR_OSVER_MAJOR (function)
macro_rules! Depcrate_um_propidlPROPSETHDR_OSVER_MAJOR {
() => {
// Module: crate::um::propidl
// Provides: {"PROPSETHDR_OSVER_MAJOR"}
// Dependencies: {}
# [inline] pub fn PROPSETHDR_OSVER_MAJOR (dwOSVer : DWORD) -> BYTE { LOBYTE (LOWORD (dwOSVer)) }
};
}
