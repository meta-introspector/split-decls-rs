macro_rules! deps {
    () => {
        Raw!();
        ErrorSink!();
        Expected!();
        ParseError!();
        Span!();
    };
}

macro_rules! escape_seq_char {
    () => {
        deps!();
        # [doc = " ```bnf"] # [doc = " escape-seq-char =  %x22         ; \"    quotation mark  U+0022"] # [doc = " escape-seq-char =/ %x5C         ; \\    reverse solidus U+005C"] # [doc = " escape-seq-char =/ %x62         ; b    backspace       U+0008"] # [doc = " escape-seq-char =/ %x66         ; f    form feed       U+000C"] # [doc = " escape-seq-char =/ %x6E         ; n    line feed       U+000A"] # [doc = " escape-seq-char =/ %x72         ; r    carriage return U+000D"] # [doc = " escape-seq-char =/ %x74         ; t    tab             U+0009"] # [doc = " escape-seq-char =/ %x75 4HEXDIG ; uXXXX                U+XXXX"] # [doc = " escape-seq-char =/ %x55 8HEXDIG ; UXXXXXXXX            U+XXXXXXXX"] # [doc = " ```"] fn escape_seq_char (stream : & mut & str , raw : Raw < '_ > , error : & mut dyn ErrorSink) -> char { const EXPECTED_ESCAPES : & [Expected] = & [Expected :: Literal ("b") , Expected :: Literal ("f") , Expected :: Literal ("n") , Expected :: Literal ("r") , Expected :: Literal ("\\") , Expected :: Literal ("\"") , Expected :: Literal ("u") , Expected :: Literal ("U") ,] ; let start = stream . checkpoint () ; let Some (id) = stream . next_token () else { let offset = stream . offset_from (& raw . as_str ()) ; error . report_error (ParseError :: new ("missing escaped value") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (EXPECTED_ESCAPES) . with_unexpected (Span :: new_unchecked (offset , offset)) ,) ; return '\\' ; } ; match id { 'b' => '\u{8}' , 'f' => '\u{c}' , 'n' => '\n' , 'r' => '\r' , 't' => '\t' , 'u' => hexescape (stream , 4 , raw , error) , 'U' => hexescape (stream , 8 , raw , error) , '\\' => '\\' , '"' => '"' , _ => { stream . reset (& start) ; let offset = stream . offset_from (& raw . as_str ()) ; error . report_error (ParseError :: new ("missing escaped value") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (EXPECTED_ESCAPES) . with_unexpected (Span :: new_unchecked (offset , offset)) ,) ; '\\' } } }
    };
}

escape_seq_char!();