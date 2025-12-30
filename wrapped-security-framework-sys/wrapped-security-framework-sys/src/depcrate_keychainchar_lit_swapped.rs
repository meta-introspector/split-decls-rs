// Generated macro for char_lit_swapped (macro)
macro_rules! Depcrate_keychainchar_lit_swapped {
() => {
// Module: crate::keychain
// Provides: {"char_lit_swapped"}
// Dependencies: {}
macro_rules ! char_lit_swapped { ($ e : expr) => { ($ e [0] as u32) + (($ e [1] as u32) << 8) + (($ e [2] as u32) << 16) + (($ e [3] as u32) << 24) } ; }
};
}
