// Generated macro for MAKESORTLCID (function)
macro_rules! Depcrate_shared_ntdefMAKESORTLCID {
() => {
// Module: crate::shared::ntdef
// Provides: {"MAKESORTLCID"}
// Dependencies: {}
# [inline] pub fn MAKESORTLCID (lgid : LANGID , srtid : USHORT , ver : USHORT) -> LCID { MAKELCID (lgid , srtid) | ((ver as ULONG) << 20) }
};
}
