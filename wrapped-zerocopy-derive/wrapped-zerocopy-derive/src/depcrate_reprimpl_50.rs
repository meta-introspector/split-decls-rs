// Generated macro for impl_50 (impl)
macro_rules! Depcrate_reprimpl_50 {
() => {
// Module: crate::repr
// Provides: {"impl_50"}
// Dependencies: {}
impl < Prim : With < PrimitiveRepr > + Copy > ToTokens for Spanned < CompoundRepr < Prim > > { fn to_tokens (& self , ts : & mut TokenStream) { use CompoundRepr :: * ; match & self . t { C => ts . append_all (quote_spanned ! { self . span => # [repr (C)] }) , Rust => ts . append_all (quote_spanned ! { self . span => # [repr (Rust)] }) , Primitive (prim) => prim . with (| prim | Spanned :: new (prim , self . span) . to_tokens (ts)) , } } }
};
}
