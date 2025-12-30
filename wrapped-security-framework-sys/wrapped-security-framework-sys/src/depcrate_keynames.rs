// Generated macro for names (macro)
macro_rules! Depcrate_keynames {
() => {
// Module: crate::key
// Provides: {"names"}
// Dependencies: {}
# [cfg (any (feature = "OSX_10_12" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] macro_rules ! names { ($ ($ (# $ meta : literal) * $ i : ident => $ x : ident) ,*) => { extern "C" { $ ($ (# [cfg (feature = $ meta)]) * pub static $ x : SecKeyAlgorithm ;) * } # [non_exhaustive] # [derive (Copy , Clone)] pub enum Algorithm { $ ($ (# [cfg (feature = $ meta)]) * $ i ,) * } impl From < Algorithm > for SecKeyAlgorithm { fn from (m : Algorithm) -> Self { unsafe { match m { $ ($ (# [cfg (feature = $ meta)]) * Algorithm ::$ i => $ x ,) * } } } } } }
};
}
