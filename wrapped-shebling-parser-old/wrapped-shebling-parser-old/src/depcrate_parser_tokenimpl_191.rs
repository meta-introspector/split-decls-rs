// Generated macro for impl_191 (impl)
macro_rules! Depcrate_parser_tokenimpl_191 {
() => {
// Module: crate::parser::token
// Provides: {"impl_191"}
// Dependencies: {}
impl ParseToken for Keyword { fn parse_token (self , span : Span) -> ParseResult < Self > { let keyword = self . token () ; let (span , word) = recognize_string (tag_no_case (keyword)) (span) ? ; let (span , missing_space) = if self == Keyword :: Function { map (followed_by (line_space) , | space | ! space) (span) ? } else { followed_by (one_of ("[#!:")) (span) ? } ; if missing_space { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: MissingSpace) . range (& span)) ; } let (span , _) = peek (alt ((swallow (eof) , swallow (multi_trivia1) , swallow (one_of (";()<>&|")) ,))) (span) ? ; if word != keyword { context ("keywords should be lower-cased!" , fail) (span) } else { Ok ((span , self)) } } }
};
}
