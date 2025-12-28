macro_rules! deps {
    () => {
        Expected!();
        ParseError!();
        EventReceiver!();
        TokenKind!();
        Stream!();
        ErrorSink!();
        Token!();
    };
}

macro_rules! on_comment {
    () => {
        deps!();
        # [doc = " Start EOL from [`TokenKind::Comment`]"] fn on_comment (tokens : & mut Stream < '_ > , comment_token : & Token , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { receiver . comment (comment_token . span () , error) ; let Some (current_token) = tokens . next_token () else { return ; } ; match current_token . kind () { TokenKind :: Dot | TokenKind :: Equals | TokenKind :: Comma | TokenKind :: LeftSquareBracket | TokenKind :: RightSquareBracket | TokenKind :: LeftCurlyBracket | TokenKind :: RightCurlyBracket | TokenKind :: Whitespace | TokenKind :: Comment | TokenKind :: LiteralString | TokenKind :: BasicString | TokenKind :: MlLiteralString | TokenKind :: MlBasicString | TokenKind :: Atom => { let context = comment_token . span () . append (current_token . span ()) ; error . report_error (ParseError :: new ("unexpected content between comment and newline") . with_context (context) . with_expected (& [Expected :: Literal ("\n")]) . with_unexpected (current_token . span () . before ()) ,) ; receiver . error (current_token . span () , error) ; ignore_to_newline (tokens , receiver , error) ; } TokenKind :: Newline => { receiver . newline (current_token . span () , error) ; } TokenKind :: Eof => { } } }
    };
}

on_comment!()