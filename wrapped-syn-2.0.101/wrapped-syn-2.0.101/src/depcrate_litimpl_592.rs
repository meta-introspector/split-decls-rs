// Generated macro for impl_592 (impl)
macro_rules! Depcrate_litimpl_592 {
() => {
// Module: crate::lit
// Provides: {"impl_592"}
// Dependencies: {}
impl LitFloat { pub fn new (repr : & str , span : Span) -> Self { let (digits , suffix) = match value :: parse_lit_float (repr) { Some (parse) => parse , None => panic ! ("not a float literal: `{}`" , repr) , } ; let mut token : Literal = repr . parse () . unwrap () ; token . set_span (span) ; LitFloat { repr : Box :: new (LitFloatRepr { token , digits , suffix , }) , } } pub fn base10_digits (& self) -> & str { & self . repr . digits } pub fn base10_parse < N > (& self) -> Result < N > where N : FromStr , N :: Err : Display , { self . base10_digits () . parse () . map_err (| err | Error :: new (self . span () , err)) } pub fn suffix (& self) -> & str { & self . repr . suffix } pub fn span (& self) -> Span { self . repr . token . span () } pub fn set_span (& mut self , span : Span) { self . repr . token . set_span (span) ; } pub fn token (& self) -> Literal { self . repr . token . clone () } }
};
}
