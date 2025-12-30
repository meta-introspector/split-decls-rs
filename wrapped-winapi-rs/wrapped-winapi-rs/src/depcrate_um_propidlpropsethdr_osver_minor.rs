// Generated macro for PROPSETHDR_OSVER_MINOR (function)
macro_rules! Depcrate_um_propidlPROPSETHDR_OSVER_MINOR {
() => {
// Module: crate::um::propidl
// Provides: {"PROPSETHDR_OSVER_MINOR"}
// Dependencies: {}
# [inline] pub fn PROPSETHDR_OSVER_MINOR (dwOSVer : DWORD) -> BYTE { HIBYTE (LOWORD (dwOSVer)) }
};
}
