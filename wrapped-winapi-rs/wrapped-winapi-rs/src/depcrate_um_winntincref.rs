// Generated macro for INCREF (function)
macro_rules! Depcrate_um_winntINCREF {
() => {
// Module: crate::um::winnt
// Provides: {"INCREF"}
// Dependencies: {}
# [inline] pub fn INCREF (x : WORD) -> WORD { ((x & ! N_BTMASK) << N_TSHIFT) | (IMAGE_SYM_DTYPE_POINTER << N_BTSHFT) | (x & N_BTMASK) }
};
}
