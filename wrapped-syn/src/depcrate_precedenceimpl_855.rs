// Generated macro for impl_855 (impl)
macro_rules! Depcrate_precedenceimpl_855 {
() => {
// Module: crate::precedence
// Provides: {"impl_855"}
// Dependencies: {}
impl PartialOrd for Precedence { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { let this = * self as u8 ; let other = * other as u8 ; Some (this . cmp (& other)) } }
};
}
