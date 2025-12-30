// Generated macro for Parser (struct)
macro_rules! Depcrate_parserParser {
() => {
// Module: crate::parser
// Provides: {"Parser"}
// Dependencies: {}
pub struct Parser < 'a > { pub serialization : String , pub base_url : Option < & 'a Url > , pub query_encoding_override : EncodingOverride < 'a > , pub violation_fn : Option < & 'a dyn Fn (SyntaxViolation) > , pub context : Context , }
};
}
