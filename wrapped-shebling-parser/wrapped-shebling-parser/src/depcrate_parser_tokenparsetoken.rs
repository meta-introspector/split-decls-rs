// Generated macro for ParseToken (trait)
macro_rules! Depcrate_parser_tokenParseToken {
() => {
// Module: crate::parser::token
// Provides: {"ParseToken"}
// Dependencies: {}
# [doc = " Trait for [Token] types which can be directly parsed from a [ParseSpan]."] pub (super) trait ParseToken where Self : Token , { fn parse_token (self , span : ParseSpan) -> ParseResult < Self > { parse_token (self) (span) } }
};
}
