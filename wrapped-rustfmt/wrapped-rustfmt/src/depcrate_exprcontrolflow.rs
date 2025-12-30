// Generated macro for ControlFlow (struct)
macro_rules! Depcrate_exprControlFlow {
() => {
// Module: crate::expr
// Provides: {"ControlFlow"}
// Dependencies: {}
# [derive (Debug)] struct ControlFlow < 'a > { cond : Option < & 'a ast :: Expr > , block : & 'a ast :: Block , else_block : Option < & 'a ast :: Expr > , label : Option < ast :: Label > , pat : Option < & 'a ast :: Pat > , keyword : & 'a str , matcher : & 'a str , connector : & 'a str , allow_single_line : bool , nested_if : bool , span : Span , }
};
}
