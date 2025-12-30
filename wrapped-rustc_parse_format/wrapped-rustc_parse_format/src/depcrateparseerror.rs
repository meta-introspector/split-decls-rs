// Generated macro for ParseError (struct)
macro_rules! DepcrateParseError {
() => {
// Module: crate
// Provides: {"ParseError"}
// Dependencies: {}
pub struct ParseError { pub description : String , pub note : Option < String > , pub label : String , pub span : Range < usize > , pub secondary_label : Option < (String , Range < usize >) > , pub suggestion : Suggestion , }
};
}
