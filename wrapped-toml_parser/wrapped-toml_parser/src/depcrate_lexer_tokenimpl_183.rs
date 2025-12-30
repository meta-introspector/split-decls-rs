// Generated macro for impl_183 (impl)
macro_rules! Depcrate_lexer_tokenimpl_183 {
() => {
// Module: crate::lexer::token
// Provides: {"impl_183"}
// Dependencies: {}
impl TokenKind { pub const fn description (& self) -> & 'static str { match self { Self :: Dot => "`.`" , Self :: Equals => "`=`" , Self :: Comma => "`,`" , Self :: LeftSquareBracket => "`[`" , Self :: RightSquareBracket => "`]`" , Self :: LeftCurlyBracket => "`{`" , Self :: RightCurlyBracket => "`}`" , Self :: Whitespace => "whitespace" , Self :: Comment => "comment" , Self :: Newline => "newline" , Self :: LiteralString => "literal string" , Self :: BasicString => "basic string" , Self :: MlLiteralString => "multi-line literal string" , Self :: MlBasicString => "multi-line basic string" , Self :: Atom => "token" , Self :: Eof => "end-of-input" , } } pub fn encoding (& self) -> Option < Encoding > { match self { Self :: LiteralString => Some (Encoding :: LiteralString) , Self :: BasicString => Some (Encoding :: BasicString) , Self :: MlLiteralString => Some (Encoding :: MlLiteralString) , Self :: MlBasicString => Some (Encoding :: MlBasicString) , Self :: Atom | Self :: LeftSquareBracket | Self :: RightSquareBracket | Self :: Dot | Self :: Equals | Self :: Comma | Self :: RightCurlyBracket | Self :: LeftCurlyBracket | Self :: Whitespace | Self :: Newline | Self :: Comment | Self :: Eof => None , } } }
};
}
