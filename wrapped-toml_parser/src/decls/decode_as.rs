macro_rules! deps {
    () => {
        ParseError!();
        Span!();
        ScalarKind!();
        StringBuilder!();
        Raw!();
        ErrorSink!();
    };
}

macro_rules! decode_as {
    () => {
        deps!();
        pub (crate) fn decode_as < 'i > (raw : Raw < 'i > , symbol : & 'i str , kind : ScalarKind , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { output . clear () ; if ! output . push_str (symbol) { error . report_error (ParseError :: new (ALLOCATION_ERROR) . with_unexpected (Span :: new_unchecked (0 , raw . len ())) ,) ; } kind }
    };
}

decode_as!();