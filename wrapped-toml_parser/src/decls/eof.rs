macro_rules! deps {
    () => {
        EventReceiver!();
        ParseError!();
        ErrorSink!();
        Stream!();
        TokenKind!();
    };
}

macro_rules! eof {
    () => {
        deps!();
        fn eof (tokens : & mut Stream < '_ > , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink) { let Some (current_token) = tokens . next_token () else { return ; } ; match current_token . kind () { TokenKind :: Dot | TokenKind :: Equals | TokenKind :: Comma | TokenKind :: LeftSquareBracket | TokenKind :: RightSquareBracket | TokenKind :: LeftCurlyBracket | TokenKind :: RightCurlyBracket | TokenKind :: LiteralString | TokenKind :: BasicString | TokenKind :: MlLiteralString | TokenKind :: MlBasicString | TokenKind :: Atom | TokenKind :: Comment | TokenKind :: Whitespace | TokenKind :: Newline => { error . report_error (ParseError :: new ("unexpected content") . with_context (current_token . span ()) . with_expected (& []) . with_unexpected (current_token . span () . before ()) ,) ; receiver . error (current_token . span () , error) ; while let Some (current_token) = tokens . next_token () { if current_token . kind () == TokenKind :: Eof { continue ; } receiver . error (current_token . span () , error) ; } } TokenKind :: Eof => { } } }
    };
}

eof!();