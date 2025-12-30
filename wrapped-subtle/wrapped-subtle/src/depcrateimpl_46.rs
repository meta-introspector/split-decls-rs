// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl < T : ConstantTimeEq > ConstantTimeEq for CtOption < T > { # [doc = " Two `CtOption<T>`s are equal if they are both `Some` and"] # [doc = " their values are equal, or both `None`."] # [inline] fn ct_eq (& self , rhs : & CtOption < T >) -> Choice { let a = self . is_some () ; let b = rhs . is_some () ; (a & b & self . value . ct_eq (& rhs . value)) | (! a & ! b) } }
};
}
