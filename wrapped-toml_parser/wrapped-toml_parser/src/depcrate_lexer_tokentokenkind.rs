// Generated macro for TokenKind (enum)
macro_rules! Depcrate_lexer_tokenTokenKind {
() => {
// Module: crate::lexer::token
// Provides: {"TokenKind"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [repr (u8)] pub enum TokenKind { # [doc = " Either for dotted-key or float"] Dot = b'.' , # [doc = " Key-value separator"] Equals = b'=' , # [doc = " Value separator"] Comma = b',' , # [doc = " Either array or standard-table start"] LeftSquareBracket = b'[' , # [doc = " Either array or standard-table end"] RightSquareBracket = b']' , # [doc = " Inline table start"] LeftCurlyBracket = b'{' , # [doc = " Inline table end"] RightCurlyBracket = b'}' , Whitespace = WSCHAR . 0 , Comment = COMMENT_START_SYMBOL , Newline = b'\n' , LiteralString = APOSTROPHE , BasicString = QUOTATION_MARK , MlLiteralString = 1 , MlBasicString , # [doc = " Anything else"] Atom , Eof , }
};
}
