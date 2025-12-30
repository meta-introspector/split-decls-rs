// Generated macro for impl_265 (impl)
macro_rules! Depcrate_de_parser_dearrayimpl_265 {
() => {
// Module: crate::de::parser::dearray
// Provides: {"impl_265"}
// Dependencies: {}
impl < 'i > core :: ops :: Deref for DeArray < 'i > { type Target = [Spanned < DeValue < 'i > >] ; # [inline] fn deref (& self) -> & [Spanned < DeValue < 'i > >] { self . items . as_slice () } }
};
}
