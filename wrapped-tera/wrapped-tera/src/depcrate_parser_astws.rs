// Generated macro for WS (struct)
macro_rules! Depcrate_parser_astWS {
() => {
// Module: crate::parser::ast
// Provides: {"WS"}
// Dependencies: {}
# [doc = " Whether to remove the whitespace of a `{% %}` tag"] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct WS { # [doc = " `true` if the tag is `{%-`"] pub left : bool , # [doc = " `true` if the tag is `-%}`"] pub right : bool , }
};
}
