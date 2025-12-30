// Generated macro for ISPTR (function)
macro_rules! Depcrate_um_winntISPTR {
() => {
// Module: crate::um::winnt
// Provides: {"ISPTR"}
// Dependencies: {}
# [inline] pub fn ISPTR (x : WORD) -> bool { (x & N_TMASK) == (IMAGE_SYM_DTYPE_POINTER << N_BTSHFT) }
};
}
