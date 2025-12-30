// Generated macro for impl_274 (impl)
macro_rules! Depcrate_de_parser_dearrayimpl_274 {
() => {
// Module: crate::de::parser::dearray
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'i > FromIterator < Spanned < DeValue < 'i > > > for DeArray < 'i > { # [inline] # [track_caller] fn from_iter < I : IntoIterator < Item = Spanned < DeValue < 'i > > > > (iter : I) -> Self { Self { items : iter . into_iter () . collect () , array_of_tables : false , } } }
};
}
