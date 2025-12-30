// Generated macro for ISFCN (function)
macro_rules! Depcrate_um_winntISFCN {
() => {
// Module: crate::um::winnt
// Provides: {"ISFCN"}
// Dependencies: {}
# [inline] pub fn ISFCN (x : WORD) -> bool { (x & N_TMASK) == (IMAGE_SYM_DTYPE_FUNCTION << N_BTSHFT) }
};
}
