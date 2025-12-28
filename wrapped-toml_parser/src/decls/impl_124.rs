macro_rules! deps {
    () => {
        TokenKind!();
        Encoding!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl TokenKind { pub const fn description (& self) -> & 'static str { match self { Self :: Dot => "`.`" , Self :: Equals => "`=`" , Self :: Comma => "`,`" , Self :: LeftSquareBracket => "`[`" , Self :: RightSquareBracket => "`]`" , Self :: LeftCurlyBracket => "`{`" , Self :: RightCurlyBracket => "`}`" , Self :: Whitespace => "whitespace" , Self :: Comment => "comment" , Self :: Newline => "newline" , Self :: LiteralString => "literal string" , Self :: BasicString => "basic string" , Self :: MlLiteralString => "multi-line literal string" , Self :: MlBasicString => "multi-line basic string" , Self :: Atom => "token" , Self :: Eof => "end-of-input" , } } pub fn encoding (& self) -> Option < Encoding > { match self { Self :: LiteralString => Some (Encoding :: LiteralString) , Self :: BasicString => Some (Encoding :: BasicString) , Self :: MlLiteralString => Some (Encoding :: MlLiteralString) , Self :: MlBasicString => Some (Encoding :: MlBasicString) , Self :: Atom | Self :: LeftSquareBracket | Self :: RightSquareBracket | Self :: Dot | Self :: Equals | Self :: Comma | Self :: RightCurlyBracket | Self :: LeftCurlyBracket | Self :: Whitespace | Self :: Newline | Self :: Comment | Self :: Eof => None , } } }
    };
}

impl_124!();