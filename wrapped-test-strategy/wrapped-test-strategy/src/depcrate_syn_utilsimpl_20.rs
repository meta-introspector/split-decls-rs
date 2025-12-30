// Generated macro for impl_20 (impl)
macro_rules! Depcrate_syn_utilsimpl_20 {
() => {
// Module: crate::syn_utils
// Provides: {"impl_20"}
// Dependencies: {}
impl FieldKey { pub fn from_field (idx : usize , field : & Field) -> Self { Self { raw : RawFieldKey :: from_field (idx , field) , span : field . span () , } } pub fn try_from_token (token : & TokenTree) -> Option < Self > { RawFieldKey :: try_from_token (token) . map (| raw | Self { raw , span : token . span () , }) } pub fn to_dummy_ident (& self) -> Ident { Ident :: new (& format ! ("_{}" , self . raw) , self . span) } pub fn to_dummy_ident_with_span (& self , span : Span) -> Ident { Ident :: new (& format ! ("_{}" , self . raw) , span) } pub fn to_valid_ident (& self) -> Option < Ident > { self . raw . to_valid_ident () } }
};
}
