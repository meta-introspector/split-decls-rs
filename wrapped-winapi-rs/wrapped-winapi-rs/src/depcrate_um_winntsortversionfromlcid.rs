// Generated macro for SORTVERSIONFROMLCID (function)
macro_rules! Depcrate_um_winntSORTVERSIONFROMLCID {
() => {
// Module: crate::um::winnt
// Provides: {"SORTVERSIONFROMLCID"}
// Dependencies: {}
# [inline] pub fn SORTVERSIONFROMLCID (lcid : LCID) -> WORD { ((lcid >> 16) & 0xf) as WORD }
};
}
