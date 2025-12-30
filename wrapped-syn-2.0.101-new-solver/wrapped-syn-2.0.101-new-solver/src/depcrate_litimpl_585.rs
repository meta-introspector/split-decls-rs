// Generated macro for impl_585 (impl)
macro_rules! Depcrate_litimpl_585 {
() => {
// Module: crate::lit
// Provides: {"impl_585"}
// Dependencies: {}
impl LitByteStr { pub fn new (value : & [u8] , span : Span) -> Self { let mut token = Literal :: byte_string (value) ; token . set_span (span) ; LitByteStr { repr : Box :: new (LitRepr { token , suffix : Box :: < str > :: default () , }) , } } pub fn value (& self) -> Vec < u8 > { let repr = self . repr . token . to_string () ; let (value , _suffix) = value :: parse_lit_byte_str (& repr) ; value } pub fn span (& self) -> Span { self . repr . token . span () } pub fn set_span (& mut self , span : Span) { self . repr . token . set_span (span) ; } pub fn suffix (& self) -> & str { & self . repr . suffix } pub fn token (& self) -> Literal { self . repr . token . clone () } }
};
}
