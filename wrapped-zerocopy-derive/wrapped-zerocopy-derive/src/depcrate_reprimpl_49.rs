// Generated macro for impl_49 (impl)
macro_rules! Depcrate_reprimpl_49 {
() => {
// Module: crate::repr
// Provides: {"impl_49"}
// Dependencies: {}
impl < Prim , Packed > ToTokens for Repr < Prim , Packed > where Prim : With < PrimitiveRepr > + Copy , Packed : With < NonZeroU32 > + Copy , { fn to_tokens (& self , ts : & mut TokenStream) { use Repr :: * ; match self { Transparent (span) => ts . append_all (quote_spanned ! { * span => # [repr (transparent)] }) , Compound (repr , align) => { repr . to_tokens (ts) ; if let Some (align) = align { align . to_tokens (ts) ; } } } } }
};
}
