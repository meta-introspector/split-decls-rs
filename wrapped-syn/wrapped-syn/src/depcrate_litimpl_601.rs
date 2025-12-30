// Generated macro for impl_601 (impl)
macro_rules! Depcrate_litimpl_601 {
() => {
// Module: crate::lit
// Provides: {"impl_601"}
// Dependencies: {}
impl LitChar { pub fn new (value : char , span : Span) -> Self { let mut token = Literal :: character (value) ; token . set_span (span) ; LitChar { repr : Box :: new (LitRepr { token , suffix : Box :: < str > :: default () , }) , } } pub fn value (& self) -> char { let repr = self . repr . token . to_string () ; let (value , _suffix) = value :: parse_lit_char (& repr) . unwrap () ; value } pub fn span (& self) -> Span { self . repr . token . span () } pub fn set_span (& mut self , span : Span) { self . repr . token . set_span (span) ; } pub fn suffix (& self) -> & str { & self . repr . suffix } pub fn token (& self) -> Literal { self . repr . token . clone () } }
};
}
