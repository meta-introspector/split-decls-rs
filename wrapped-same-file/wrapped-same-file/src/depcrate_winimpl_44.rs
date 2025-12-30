// Generated macro for impl_44 (impl)
macro_rules! Depcrate_winimpl_44 {
() => {
// Module: crate::win
// Provides: {"impl_44"}
// Dependencies: {}
impl PartialEq for Handle { fn eq (& self , other : & Handle) -> bool { if self as * const Handle == other as * const Handle { return true ; } else if self . key . is_none () || other . key . is_none () { return false ; } self . key == other . key } }
};
}
