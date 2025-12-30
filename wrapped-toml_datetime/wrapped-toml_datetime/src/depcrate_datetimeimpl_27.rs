// Generated macro for impl_27 (impl)
macro_rules! Depcrate_datetimeimpl_27 {
() => {
// Module: crate::datetime
// Provides: {"impl_27"}
// Dependencies: {}
impl < 's > Lexer < 's > { fn new (input : & 's str) -> Self { Self { stream : input } } fn unknown (& mut self) -> Option < Token < 's > > { let remaining = self . stream . len () ; if remaining == 0 { return None ; } let raw = self . stream ; self . stream = & self . stream [remaining .. remaining] ; Some (Token { kind : TokenKind :: Unknown , raw , }) } }
};
}
