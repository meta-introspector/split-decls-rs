macro_rules! deps {
    () => {
        ErrorSink!();
        Expected!();
        ParseError!();
        EventReceiver!();
        Stream!();
        TokenKind!();
    };
}

macro_rules! ws_comment_newline {
    () => {
        deps!();
        # [doc = " Parse EOL decor, if present"] # [doc = ""] # [doc = " ```bnf"] # [doc = " toml = expression *( newline expression )"] # [doc = ""] # [doc = " expression =  ws [ on_comment ]"] # [doc = " expression =/ ws keyval ws [ on_comment ]"] # [doc = " expression =/ ws table ws [ on_comment ]"] # [doc = ""] # [doc = " ;; Whitespace"] # [doc = ""] # [doc = " ws = *wschar"] # [doc = " wschar =  %x20  ; Space"] # [doc = " wschar =/ %x09  ; Horizontal tab"] # [doc = ""] # [doc = " ;; Newline"] # [doc = ""] # [doc = " newline =  %x0A     ; LF"] # [doc = " newline =/ %x0D.0A  ; CRLF"] # [doc = ""] # [doc = " ;; Comment"] # [doc = ""] # [doc = " comment = comment-start-symbol *non-eol"] # [doc = " ```"] fn ws_comment_newline (tokens : & mut Stream < '_ > , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { let mut first = None ; while let Some (current_token) = tokens . next_token () { let first = first . get_or_insert (current_token . span ()) ; match current_token . kind () { TokenKind :: Dot | TokenKind :: Equals | TokenKind :: Comma | TokenKind :: LeftSquareBracket | TokenKind :: RightSquareBracket | TokenKind :: LeftCurlyBracket | TokenKind :: RightCurlyBracket | TokenKind :: LiteralString | TokenKind :: BasicString | TokenKind :: MlLiteralString | TokenKind :: MlBasicString | TokenKind :: Atom => { let context = first . append (current_token . span ()) ; error . report_error (ParseError :: new ("unexpected key or value") . with_context (context) . with_expected (& [Expected :: Literal ("\n") , Expected :: Literal ("#")]) . with_unexpected (current_token . span () . before ()) ,) ; receiver . error (current_token . span () , error) ; ignore_to_newline (tokens , receiver , error) ; break ; } TokenKind :: Comment => { on_comment (tokens , current_token , receiver , error) ; break ; } TokenKind :: Whitespace => { receiver . whitespace (current_token . span () , error) ; continue ; } TokenKind :: Newline => { receiver . newline (current_token . span () , error) ; break ; } TokenKind :: Eof => { break ; } } } }
    };
}

ws_comment_newline!();