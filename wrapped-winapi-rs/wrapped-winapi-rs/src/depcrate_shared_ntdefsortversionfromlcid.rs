// Generated macro for SORTVERSIONFROMLCID (function)
macro_rules! Depcrate_shared_ntdefSORTVERSIONFROMLCID {
() => {
// Module: crate::shared::ntdef
// Provides: {"SORTVERSIONFROMLCID"}
// Dependencies: {}
# [inline] pub fn SORTVERSIONFROMLCID (lcid : LCID) -> USHORT { ((lcid >> 16) & 0xf) as USHORT }
};
}
