// Generated macro for impl_104 (impl)
macro_rules! Depcrate_identimpl_104 {
() => {
// Module: crate::ident
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a > From < Cow < 'a , str > > for Ident { fn from (s : Cow < 'a , str >) -> Self { Ident :: new (Term :: intern (& s) , Span :: default ()) } }
};
}
