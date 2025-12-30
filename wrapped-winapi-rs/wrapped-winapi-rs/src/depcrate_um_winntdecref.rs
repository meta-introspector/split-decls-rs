// Generated macro for DECREF (function)
macro_rules! Depcrate_um_winntDECREF {
() => {
// Module: crate::um::winnt
// Provides: {"DECREF"}
// Dependencies: {}
# [inline] pub fn DECREF (x : WORD) -> WORD { ((x >> N_TSHIFT) & ! N_BTMASK) | (x & N_BTMASK) }
};
}
