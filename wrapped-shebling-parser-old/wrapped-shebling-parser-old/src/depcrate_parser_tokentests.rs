// Generated macro for tests (module)
macro_rules! Depcrate_parser_tokentests {
() => {
// Module: crate::parser::token
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_parse_control_op_token () { let mut parser = token (ControlOp :: Semi) ; assert_parse ! (parser (";") , ControlOp :: Semi) ; assert_parse ! (parser (";;") => Err (1 , 1)) ; } # [test] fn test_parse_keyword_token () { let mut parser = token (Keyword :: If) ; assert_parse ! (parser ("if[") => Err ((1 , 3) , Diags : [((1 , 3) , ParseDiagnosticKind :: MissingSpace)])) ; assert_parse ! (parser ("if") , Keyword :: If) ; assert_parse ! (parser ("if ") => " " , Keyword :: If) ; assert_parse ! (parser ("if;") => ";" , Keyword :: If) ; assert_parse ! (parser ("If") => Err ((1 , 3) , Notes : [((1 , 3) , "keywords should be lower-cased")])) ; parser = token (Keyword :: Function) ; assert_parse ! (parser ("function foo") => " foo" , Keyword :: Function) ; assert_parse ! (parser ("function") , Keyword :: Function , [((1 , 9) , ParseDiagnosticKind :: MissingSpace)]) ; } }
};
}
