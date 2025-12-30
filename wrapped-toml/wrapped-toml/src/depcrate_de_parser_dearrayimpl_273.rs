// Generated macro for impl_273 (impl)
macro_rules! Depcrate_de_parser_dearrayimpl_273 {
() => {
// Module: crate::de::parser::dearray
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'i > IntoIterator for DeArray < 'i > { type Item = Spanned < DeValue < 'i > > ; type IntoIter = alloc :: vec :: IntoIter < Spanned < DeValue < 'i > > > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . items . into_iter () } }
};
}
