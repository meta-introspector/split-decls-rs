macro_rules! deps {
    () => {
        TokenKind!();
        Token!();
        Stream!();
    };
}

macro_rules! process_token {
    () => {
        deps!();
        fn process_token (peek_byte : u8 , stream : & mut Stream < '_ >) -> Token { let token = match peek_byte { b'.' => lex_ascii_char (stream , TokenKind :: Dot) , b'=' => lex_ascii_char (stream , TokenKind :: Equals) , b',' => lex_ascii_char (stream , TokenKind :: Comma) , b'[' => lex_ascii_char (stream , TokenKind :: LeftSquareBracket) , b']' => lex_ascii_char (stream , TokenKind :: RightSquareBracket) , b'{' => lex_ascii_char (stream , TokenKind :: LeftCurlyBracket) , b'}' => lex_ascii_char (stream , TokenKind :: RightCurlyBracket) , b' ' => lex_whitespace (stream) , b'\t' => lex_whitespace (stream) , b'#' => lex_comment (stream) , b'\r' => lex_crlf (stream) , b'\n' => lex_ascii_char (stream , TokenKind :: Newline) , b'\'' => { if stream . starts_with (ML_LITERAL_STRING_DELIM) { lex_ml_literal_string (stream) } else { lex_literal_string (stream) } } b'"' => { if stream . starts_with (ML_BASIC_STRING_DELIM) { lex_ml_basic_string (stream) } else { lex_basic_string (stream) } } _ => lex_atom (stream) , } ; token }
    };
}

process_token!();