// Generated macro for impl_54 (impl)
macro_rules! Depcrate_abiimpl_54 {
() => {
// Module: crate::abi
// Provides: {"impl_54"}
// Dependencies: {}
impl LayoutShape { # [doc = " Returns `true` if the layout corresponds to an unsized type."] # [inline] pub fn is_unsized (& self) -> bool { self . abi . is_unsized () } # [inline] pub fn is_sized (& self) -> bool { ! self . abi . is_unsized () } # [doc = " Returns `true` if the type is sized and a 1-ZST (meaning it has size 0 and alignment 1)."] pub fn is_1zst (& self) -> bool { self . is_sized () && self . size . bits () == 0 && self . abi_align == 1 } }
};
}
