// Generated macro for MAKELCID (function)
macro_rules! Depcrate_um_winntMAKELCID {
() => {
// Module: crate::um::winnt
// Provides: {"MAKELCID"}
// Dependencies: {}
# [inline] pub fn MAKELCID (lgid : LANGID , srtid : WORD) -> LCID { ((srtid as DWORD) << 16) | (lgid as DWORD) }
};
}
