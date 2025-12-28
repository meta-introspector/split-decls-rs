macro_rules! deps {
    () => {
        Expected!();
        ErrorSink!();
        EventReceiver!();
        TokenKind!();
        Stream!();
        ParseError!();
    };
}

macro_rules! value {
    () => {
        deps!();
        # [doc = " Parse a value"] # [doc = ""] # [doc = " ```abnf"] # [doc = " val = string / boolean / array / inline-table / date-time / float / integer"] # [doc = " ```"] fn value (tokens : & mut Stream < '_ > , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink) { let Some (current_token) = tokens . next_token () else { let previous_span = tokens . previous_tokens () . find (| t | { ! matches ! (t . kind () , TokenKind :: Whitespace | TokenKind :: Comment | TokenKind :: Newline | TokenKind :: Eof) }) . map (| t | t . span ()) . unwrap_or_default () ; error . report_error (ParseError :: new ("missing value") . with_context (previous_span) . with_expected (& [Expected :: Description ("value")]) . with_unexpected (previous_span . after ()) ,) ; return ; } ; match current_token . kind () { TokenKind :: Comment | TokenKind :: Comma | TokenKind :: Newline | TokenKind :: Eof | TokenKind :: Whitespace => { let fake_key = current_token . span () . before () ; let encoding = None ; receiver . scalar (fake_key , encoding , error) ; seek (tokens , - 1) ; } TokenKind :: Equals => { error . report_error (ParseError :: new ("extra `=`") . with_context (current_token . span ()) . with_expected (& []) . with_unexpected (current_token . span ()) ,) ; receiver . error (current_token . span () , error) ; value (tokens , receiver , error) ; } TokenKind :: LeftCurlyBracket => { on_inline_table_open (tokens , current_token , receiver , error) ; } TokenKind :: RightCurlyBracket => { error . report_error (ParseError :: new ("missing inline table opening") . with_context (current_token . span ()) . with_expected (& [Expected :: Literal ("{")]) . with_unexpected (current_token . span () . before ()) ,) ; let _ = receiver . inline_table_open (current_token . span () . before () , error) ; receiver . inline_table_close (current_token . span () , error) ; } TokenKind :: LeftSquareBracket => { on_array_open (tokens , current_token , receiver , error) ; } TokenKind :: RightSquareBracket => { error . report_error (ParseError :: new ("missing array opening") . with_context (current_token . span ()) . with_expected (& [Expected :: Literal ("[")]) . with_unexpected (current_token . span () . before ()) ,) ; let _ = receiver . array_open (current_token . span () . before () , error) ; receiver . array_close (current_token . span () , error) ; } TokenKind :: LiteralString | TokenKind :: BasicString | TokenKind :: MlLiteralString | TokenKind :: MlBasicString | TokenKind :: Dot | TokenKind :: Atom => { on_scalar (tokens , current_token , receiver , error) ; } } }
    };
}

value!();