// Generated macro for SORTIDFROMLCID (function)
macro_rules! Depcrate_um_winntSORTIDFROMLCID {
() => {
// Module: crate::um::winnt
// Provides: {"SORTIDFROMLCID"}
// Dependencies: {}
# [inline] pub fn SORTIDFROMLCID (lcid : LCID) -> WORD { ((lcid >> 16) & 0xf) as WORD }
};
}
