// Generated macro for impl_839 (impl)
macro_rules! Depcrate_precedenceimpl_839 {
() => {
// Module: crate::precedence
// Provides: {"impl_839"}
// Dependencies: {}
impl PartialOrd for Precedence { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { let this = * self as u8 ; let other = * other as u8 ; Some (this . cmp (& other)) } }
};
}
