// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
# [doc = " `Ordering` is `#[repr(i8)]` making it possible to leverage `i8::ct_eq`."] impl ConstantTimeEq for cmp :: Ordering { # [inline] fn ct_eq (& self , other : & Self) -> Choice { (* self as i8) . ct_eq (& (* other as i8)) } }
};
}
