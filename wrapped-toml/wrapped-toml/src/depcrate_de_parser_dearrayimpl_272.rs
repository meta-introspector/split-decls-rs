// Generated macro for impl_272 (impl)
macro_rules! Depcrate_de_parser_dearrayimpl_272 {
() => {
// Module: crate::de::parser::dearray
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'a , 'i > IntoIterator for & 'a DeArray < 'i > { type Item = & 'a Spanned < DeValue < 'i > > ; type IntoIter = core :: slice :: Iter < 'a , Spanned < DeValue < 'i > > > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
