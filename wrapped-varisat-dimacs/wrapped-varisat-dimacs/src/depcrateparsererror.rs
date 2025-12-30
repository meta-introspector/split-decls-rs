// Generated macro for ParserError (enum)
macro_rules! DepcrateParserError {
() => {
// Module: crate
// Provides: {"ParserError"}
// Dependencies: {}
# [doc = " Possible errors while parsing a DIMACS CNF formula."] # [derive (Debug , Error)] pub enum ParserError { # [error ("line {}: Unexpected character in DIMACS CNF input: '{}'" , line , unexpected)] UnexpectedInput { line : usize , unexpected : char } , # [error ("line {}: Literal index is too large: {}{}..." , line , index , final_digit)] LiteralTooLarge { line : usize , index : usize , final_digit : usize , } , # [error ("line {}: Invalid header syntax: {}" , line , header)] InvalidHeader { line : usize , header : String } , # [error ("line {}: Unterminated clause" , line)] UnterminatedClause { line : usize } , # [error ("Formula has {} variables while the header specifies {} variables" , var_count , header_var_count)] VarCount { var_count : usize , header_var_count : usize , } , # [error ("Formula has {} clauses while the header specifies {} clauses" , clause_count , header_clause_count)] ClauseCount { clause_count : usize , header_clause_count : usize , } , # [error ("Parser invoked after a previous error")] PreviousError , }
};
}
