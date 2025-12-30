// Generated macro for impl_51 (impl)
macro_rules! Depcrate_reprimpl_51 {
() => {
// Module: crate::repr
// Provides: {"impl_51"}
// Dependencies: {}
impl ToTokens for Spanned < PrimitiveRepr > { fn to_tokens (& self , ts : & mut TokenStream) { use PrimitiveRepr :: * ; match self . t { U8 => ts . append_all (quote_spanned ! { self . span => # [repr (u8)] }) , U16 => ts . append_all (quote_spanned ! { self . span => # [repr (u16)] }) , U32 => ts . append_all (quote_spanned ! { self . span => # [repr (u32)] }) , U64 => ts . append_all (quote_spanned ! { self . span => # [repr (u64)] }) , U128 => ts . append_all (quote_spanned ! { self . span => # [repr (u128)] }) , Usize => ts . append_all (quote_spanned ! { self . span => # [repr (usize)] }) , I8 => ts . append_all (quote_spanned ! { self . span => # [repr (i8)] }) , I16 => ts . append_all (quote_spanned ! { self . span => # [repr (i16)] }) , I32 => ts . append_all (quote_spanned ! { self . span => # [repr (i32)] }) , I64 => ts . append_all (quote_spanned ! { self . span => # [repr (i64)] }) , I128 => ts . append_all (quote_spanned ! { self . span => # [repr (i128)] }) , Isize => ts . append_all (quote_spanned ! { self . span => # [repr (isize)] }) , } } }
};
}
