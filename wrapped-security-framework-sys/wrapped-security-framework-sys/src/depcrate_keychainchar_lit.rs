// Generated macro for char_lit (macro)
macro_rules! Depcrate_keychainchar_lit {
() => {
// Module: crate::keychain
// Provides: {"char_lit"}
// Dependencies: {}
# [doc = " Like Apple's headers, it assumes Little Endian,"] # [doc = " as there are no supported Big Endian machines any more :("] macro_rules ! char_lit { ($ e : expr) => { ($ e [3] as u32) + (($ e [2] as u32) << 8) + (($ e [1] as u32) << 16) + (($ e [0] as u32) << 24) } ; }
};
}
