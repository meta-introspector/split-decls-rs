macro_rules! deps {
    () => {
        TokenKind!();
        Encoding!();
        Expected!();
        Stream!();
        EventReceiver!();
        ParseError!();
        ErrorSink!();
    };
}

macro_rules! key {
    () => {
        deps!();
        # [doc = " Parse a TOML key"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Key-Value pairs"] # [doc = ""] # [doc = " key = simple-key / dotted-key"] # [doc = " simple-key = quoted-key / unquoted-key"] # [doc = ""] # [doc = " quoted-key = basic-string / literal-string"] # [doc = " dotted-key = simple-key 1*( dot-sep simple-key )"] # [doc = ""] # [doc = " dot-sep   = ws %x2E ws  ; . Period"] # [doc = " ```"] fn key (tokens : & mut Stream < '_ > , invalid_description : & 'static str , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) -> bool { while let Some (current_token) = tokens . next_token () { let encoding = match current_token . kind () { TokenKind :: RightSquareBracket | TokenKind :: Comment | TokenKind :: Equals | TokenKind :: Comma | TokenKind :: LeftSquareBracket | TokenKind :: LeftCurlyBracket | TokenKind :: RightCurlyBracket | TokenKind :: Newline | TokenKind :: Eof => { let fake_key = current_token . span () . before () ; let encoding = None ; receiver . simple_key (fake_key , encoding , error) ; seek (tokens , - 1) ; return false ; } TokenKind :: Whitespace => { receiver . whitespace (current_token . span () , error) ; continue ; } TokenKind :: Dot => { let fake_key = current_token . span () . before () ; let encoding = None ; receiver . simple_key (fake_key , encoding , error) ; receiver . key_sep (current_token . span () , error) ; continue ; } TokenKind :: LiteralString => Some (Encoding :: LiteralString) , TokenKind :: BasicString => Some (Encoding :: BasicString) , TokenKind :: MlLiteralString => Some (Encoding :: MlLiteralString) , TokenKind :: MlBasicString => Some (Encoding :: MlBasicString) , TokenKind :: Atom => None , } ; receiver . simple_key (current_token . span () , encoding , error) ; return opt_dot_keys (tokens , receiver , error) ; } let previous_span = tokens . previous_tokens () . find (| t | { ! matches ! (t . kind () , TokenKind :: Whitespace | TokenKind :: Comment | TokenKind :: Newline | TokenKind :: Eof) }) . map (| t | t . span ()) . unwrap_or_default () ; error . report_error (ParseError :: new (invalid_description) . with_context (previous_span) . with_expected (& [Expected :: Description ("key")]) . with_unexpected (previous_span . after ()) ,) ; false }
    };
}

key!();