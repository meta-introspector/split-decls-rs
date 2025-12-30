// Generated macro for SORTIDFROMLCID (function)
macro_rules! Depcrate_shared_ntdefSORTIDFROMLCID {
() => {
// Module: crate::shared::ntdef
// Provides: {"SORTIDFROMLCID"}
// Dependencies: {}
# [inline] pub fn SORTIDFROMLCID (lcid : LCID) -> USHORT { ((lcid >> 16) & 0xf) as USHORT }
};
}
