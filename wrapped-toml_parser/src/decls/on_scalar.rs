macro_rules! deps {
    () => {
        Stream!();
        Token!();
        Encoding!();
        EventReceiver!();
        ErrorSink!();
        TokenKind!();
    };
}

macro_rules! on_scalar {
    () => {
        deps!();
        # [doc = " Parse a scalar value"] # [doc = ""] # [doc = " ```abnf"] # [doc = " val = string / boolean / array / inline-table / date-time / float / integer"] # [doc = " ```"] fn on_scalar (tokens : & mut Stream < '_ > , scalar : & Token , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { let mut span = scalar . span () ; let encoding = match scalar . kind () { TokenKind :: Comment | TokenKind :: Comma | TokenKind :: Newline | TokenKind :: Eof | TokenKind :: Whitespace | TokenKind :: Equals | TokenKind :: LeftCurlyBracket | TokenKind :: RightCurlyBracket | TokenKind :: LeftSquareBracket | TokenKind :: RightSquareBracket => { unreachable ! () } TokenKind :: LiteralString => Some (Encoding :: LiteralString) , TokenKind :: BasicString => Some (Encoding :: BasicString) , TokenKind :: MlLiteralString => Some (Encoding :: MlLiteralString) , TokenKind :: MlBasicString => Some (Encoding :: MlBasicString) , TokenKind :: Dot | TokenKind :: Atom => { while let Some (next_token) = tokens . first () { match next_token . kind () { TokenKind :: Comment | TokenKind :: Comma | TokenKind :: Newline | TokenKind :: Eof | TokenKind :: Equals | TokenKind :: LeftCurlyBracket | TokenKind :: RightCurlyBracket | TokenKind :: LeftSquareBracket | TokenKind :: RightSquareBracket | TokenKind :: LiteralString | TokenKind :: BasicString | TokenKind :: MlLiteralString | TokenKind :: MlBasicString => { break ; } TokenKind :: Whitespace => { if let Some (second) = tokens . get (1) { if second . kind () == TokenKind :: Atom { span = span . append (second . span ()) ; let _ = tokens . next_slice (2) ; continue ; } } break ; } TokenKind :: Dot | TokenKind :: Atom => { span = span . append (next_token . span ()) ; let _ = tokens . next_token () ; } } } None } } ; receiver . scalar (span , encoding , error) ; }
    };
}

on_scalar!();