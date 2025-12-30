// Generated macro for impl_275 (impl)
macro_rules! Depcrate_parser_eventimpl_275 {
() => {
// Module: crate::parser::event
// Provides: {"impl_275"}
// Dependencies: {}
impl Event { pub fn new_unchecked (kind : EventKind , encoding : Option < Encoding > , span : Span) -> Self { Self { kind , encoding , span , } } # [inline (always)] pub fn kind (& self) -> EventKind { self . kind } # [inline (always)] pub fn encoding (& self) -> Option < Encoding > { self . encoding } # [inline (always)] pub fn span (& self) -> Span { self . span } }
};
}
