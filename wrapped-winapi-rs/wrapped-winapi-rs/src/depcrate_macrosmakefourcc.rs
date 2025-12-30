// Generated macro for MAKEFOURCC (macro)
macro_rules! Depcrate_macrosMAKEFOURCC {
() => {
// Module: crate::macros
// Provides: {"MAKEFOURCC"}
// Dependencies: {}
macro_rules ! MAKEFOURCC { ($ a : expr , $ b : expr , $ c : expr , $ d : expr) => { ($ a as u32) | (($ b as u32) << 8) | (($ c as u32) << 16) | (($ d as u32) << 24) } }
};
}
