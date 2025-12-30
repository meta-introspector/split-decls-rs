// Generated macro for impl_608 (impl)
macro_rules! Depcrate_litimpl_608 {
() => {
// Module: crate::lit
// Provides: {"impl_608"}
// Dependencies: {}
impl LitBool { pub fn new (value : bool , span : Span) -> Self { LitBool { value , span } } pub fn value (& self) -> bool { self . value } pub fn span (& self) -> Span { self . span } pub fn set_span (& mut self , span : Span) { self . span = span ; } pub fn token (& self) -> Ident { let s = if self . value { "true" } else { "false" } ; Ident :: new (s , self . span) } }
};
}
