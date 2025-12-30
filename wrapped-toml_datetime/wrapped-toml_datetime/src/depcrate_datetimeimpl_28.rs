// Generated macro for impl_28 (impl)
macro_rules! Depcrate_datetimeimpl_28 {
() => {
// Module: crate::datetime
// Provides: {"impl_28"}
// Dependencies: {}
impl < 's > Iterator for Lexer < 's > { type Item = Token < 's > ; fn next (& mut self) -> Option < Self :: Item > { let (kind , end) = match self . stream . as_bytes () . first () ? { b'0' ..= b'9' => { let end = self . stream . as_bytes () . iter () . position (| b | ! b . is_ascii_digit ()) . unwrap_or (self . stream . len ()) ; (TokenKind :: Digits , end) } b'-' => (TokenKind :: Dash , 1) , b':' => (TokenKind :: Colon , 1) , b'T' | b't' => (TokenKind :: T , 1) , b' ' => (TokenKind :: Space , 1) , b'Z' | b'z' => (TokenKind :: Z , 1) , b'+' => (TokenKind :: Plus , 1) , b'.' => (TokenKind :: Dot , 1) , _ => (TokenKind :: Unknown , self . stream . len ()) , } ; let (raw , rest) = self . stream . split_at (end) ; self . stream = rest ; Some (Token { kind , raw }) } }
};
}
