macro_rules! deps {
    () => {
        ErrorSink!();
        ParseError!();
        EventReceiver!();
        TokenKind!();
        Expected!();
        Stream!();
        Encoding!();
    };
}

macro_rules! simple_key {
    () => {
        deps!();
        # [doc = " Parse a TOML simple key"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Key-Value pairs"] # [doc = ""] # [doc = " simple-key = quoted-key / unquoted-key"] # [doc = ""] # [doc = " quoted-key = basic-string / literal-string"] # [doc = " ```"] fn simple_key (tokens : & mut Stream < '_ > , invalid_description : & 'static str , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { let Some (current_token) = tokens . next_token () else { let previous_span = tokens . previous_tokens () . find (| t | { ! matches ! (t . kind () , TokenKind :: Whitespace | TokenKind :: Comment | TokenKind :: Newline | TokenKind :: Eof) }) . map (| t | t . span ()) . unwrap_or_default () ; error . report_error (ParseError :: new (invalid_description) . with_context (previous_span) . with_expected (& [Expected :: Description ("key")]) . with_unexpected (previous_span . after ()) ,) ; return ; } ; const EXPECTED_KEYS : [Expected ; 3] = [Expected :: Description (Encoding :: LiteralString . description ()) , Expected :: Description (Encoding :: BasicString . description ()) , Expected :: Description (UNQUOTED_STRING) ,] ; let kind = match current_token . kind () { TokenKind :: Dot | TokenKind :: RightSquareBracket | TokenKind :: Comment | TokenKind :: Equals | TokenKind :: Comma | TokenKind :: LeftSquareBracket | TokenKind :: LeftCurlyBracket | TokenKind :: RightCurlyBracket | TokenKind :: Newline | TokenKind :: Eof | TokenKind :: Whitespace => { on_missing_key (tokens , current_token , invalid_description , receiver , error) ; return ; } TokenKind :: LiteralString => Some (Encoding :: LiteralString) , TokenKind :: BasicString => Some (Encoding :: BasicString) , TokenKind :: MlLiteralString => { error . report_error (ParseError :: new (invalid_description) . with_context (current_token . span ()) . with_expected (& EXPECTED_KEYS) . with_unexpected (current_token . span ()) ,) ; Some (Encoding :: MlLiteralString) } TokenKind :: MlBasicString => { error . report_error (ParseError :: new (invalid_description) . with_context (current_token . span ()) . with_expected (& EXPECTED_KEYS) . with_unexpected (current_token . span ()) ,) ; Some (Encoding :: MlBasicString) } TokenKind :: Atom => None , } ; receiver . simple_key (current_token . span () , kind , error) ; }
    };
}

simple_key!();