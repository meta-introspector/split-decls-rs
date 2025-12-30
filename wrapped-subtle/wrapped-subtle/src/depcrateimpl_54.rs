// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl ConstantTimeGreater for cmp :: Ordering { # [inline] fn ct_gt (& self , other : & Self) -> Choice { let a = (* self as i8) + 1 ; let b = (* other as i8) + 1 ; (a as u8) . ct_gt (& (b as u8)) } }
};
}
