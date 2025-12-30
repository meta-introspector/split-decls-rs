// Generated macro for impl_599 (impl)
macro_rules! Depcrate_litimpl_599 {
() => {
// Module: crate::lit
// Provides: {"impl_599"}
// Dependencies: {}
impl LitCStr { pub fn new (value : & CStr , span : Span) -> Self { let mut token = Literal :: c_string (value) ; token . set_span (span) ; LitCStr { repr : Box :: new (LitRepr { token , suffix : Box :: < str > :: default () , }) , } } pub fn value (& self) -> CString { let repr = self . repr . token . to_string () ; let (value , _suffix) = value :: parse_lit_c_str (& repr) . unwrap () ; value } pub fn span (& self) -> Span { self . repr . token . span () } pub fn set_span (& mut self , span : Span) { self . repr . token . set_span (span) ; } pub fn suffix (& self) -> & str { & self . repr . suffix } pub fn token (& self) -> Literal { self . repr . token . clone () } }
};
}
