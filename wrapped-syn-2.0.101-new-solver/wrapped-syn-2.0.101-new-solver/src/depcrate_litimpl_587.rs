// Generated macro for impl_587 (impl)
macro_rules! Depcrate_litimpl_587 {
() => {
// Module: crate::lit
// Provides: {"impl_587"}
// Dependencies: {}
impl LitByte { pub fn new (value : u8 , span : Span) -> Self { let mut token = Literal :: u8_suffixed (value) ; token . set_span (span) ; LitByte { repr : Box :: new (LitRepr { token , suffix : Box :: < str > :: default () , }) , } } pub fn value (& self) -> u8 { let repr = self . repr . token . to_string () ; let (value , _suffix) = value :: parse_lit_byte (& repr) ; value } pub fn span (& self) -> Span { self . repr . token . span () } pub fn set_span (& mut self , span : Span) { self . repr . token . set_span (span) ; } pub fn suffix (& self) -> & str { & self . repr . suffix } pub fn token (& self) -> Literal { self . repr . token . clone () } }
};
}
