// Generated macro for MAKESORTLCID (function)
macro_rules! Depcrate_um_winntMAKESORTLCID {
() => {
// Module: crate::um::winnt
// Provides: {"MAKESORTLCID"}
// Dependencies: {}
# [inline] pub fn MAKESORTLCID (lgid : LANGID , srtid : WORD , ver : WORD) -> LCID { MAKELCID (lgid , srtid) | ((ver as DWORD) << 20) }
};
}
