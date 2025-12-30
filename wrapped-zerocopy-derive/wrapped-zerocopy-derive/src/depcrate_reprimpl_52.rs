// Generated macro for impl_52 (impl)
macro_rules! Depcrate_reprimpl_52 {
() => {
// Module: crate::repr
// Provides: {"impl_52"}
// Dependencies: {}
impl < Packed : With < NonZeroU32 > + Copy > ToTokens for Spanned < AlignRepr < Packed > > { fn to_tokens (& self , ts : & mut TokenStream) { use AlignRepr :: * ; let to_index = | n : NonZeroU32 | syn :: Index { index : n . get () , span : self . span } ; match self . t { Packed (n) => n . with (| n | { let n = to_index (n) ; ts . append_all (quote_spanned ! { self . span => # [repr (packed (# n))] }) }) , Align (n) => { let n = to_index (n) ; ts . append_all (quote_spanned ! { self . span => # [repr (align (# n))] }) } } } }
};
}
