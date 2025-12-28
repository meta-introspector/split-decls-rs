macro_rules! deps {
    () => {
        Stream!();
        ErrorSink!();
        EventReceiver!();
        TokenKind!();
    };
}

macro_rules! ignore_to_newline {
    () => {
        deps!();
        # [cold] fn ignore_to_newline (tokens : & mut Stream < '_ > , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { while let Some (current_token) = tokens . next_token () { match current_token . kind () { TokenKind :: Dot | TokenKind :: Equals | TokenKind :: Comma | TokenKind :: LeftSquareBracket | TokenKind :: RightSquareBracket | TokenKind :: LeftCurlyBracket | TokenKind :: RightCurlyBracket | TokenKind :: LiteralString | TokenKind :: BasicString | TokenKind :: MlLiteralString | TokenKind :: MlBasicString | TokenKind :: Atom => { receiver . error (current_token . span () , error) ; } TokenKind :: Comment => { on_comment (tokens , current_token , receiver , error) ; break ; } TokenKind :: Whitespace => { receiver . whitespace (current_token . span () , error) ; } TokenKind :: Newline => { receiver . newline (current_token . span () , error) ; break ; } TokenKind :: Eof => { break ; } } } }
    };
}

ignore_to_newline!()