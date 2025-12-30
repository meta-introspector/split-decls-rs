// Generated macro for if_loom (macro)
macro_rules! Depcrate_macros_loomif_loom {
() => {
// Module: crate::macros::loom
// Provides: {"if_loom"}
// Dependencies: {}
macro_rules ! if_loom { ($ ($ t : tt) *) => { { # [cfg (loom)] { $ ($ t) * } } } }
};
}
