macro_rules! deps {
    () => {
        ParseBuffer!();
        Unexpected!();
        TokenBuffer!();
    };
}

macro_rules! tokens_to_parse_buffer {
    () => {
        deps!();
        fn tokens_to_parse_buffer (tokens : & TokenBuffer) -> ParseBuffer { let scope = Span :: call_site () ; let cursor = tokens . begin () ; let unexpected = Rc :: new (Cell :: new (Unexpected :: None)) ; new_parse_buffer (scope , cursor , unexpected) }
    };
}

tokens_to_parse_buffer!()