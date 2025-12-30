// Generated macro for MAKELCID (function)
macro_rules! Depcrate_shared_ntdefMAKELCID {
() => {
// Module: crate::shared::ntdef
// Provides: {"MAKELCID"}
// Dependencies: {}
# [inline] pub fn MAKELCID (lgid : LANGID , srtid : USHORT) -> LCID { ((srtid as ULONG) << 16) | (lgid as ULONG) }
};
}
