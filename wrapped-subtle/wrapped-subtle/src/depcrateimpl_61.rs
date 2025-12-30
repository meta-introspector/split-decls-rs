// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl ConstantTimeLess for cmp :: Ordering { # [inline] fn ct_lt (& self , other : & Self) -> Choice { let a = (* self as i8) + 1 ; let b = (* other as i8) + 1 ; (a as u8) . ct_lt (& (b as u8)) } }
};
}
