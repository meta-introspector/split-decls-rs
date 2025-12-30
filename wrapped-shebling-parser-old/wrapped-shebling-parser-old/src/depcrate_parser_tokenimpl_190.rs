// Generated macro for impl_190 (impl)
macro_rules! Depcrate_parser_tokenimpl_190 {
() => {
// Module: crate::parser::token
// Provides: {"impl_190"}
// Dependencies: {}
impl ParseToken for ControlOp { fn parse_token (self , mut span : Span) -> ParseResult < Self > { if let ControlOp :: Semi = self { (span , _) = not (token (ControlOp :: DSemi)) (span) ? ; } parse_token (self) (span) } }
};
}
