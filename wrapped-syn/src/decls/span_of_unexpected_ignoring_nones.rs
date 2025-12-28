macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! span_of_unexpected_ignoring_nones {
    () => {
        deps!();
        fn span_of_unexpected_ignoring_nones (mut cursor : Cursor) -> Option < (Span , Delimiter) > { if cursor . eof () { return None ; } while let Some ((inner , _span , rest)) = cursor . group (Delimiter :: None) { if let Some (unexpected) = span_of_unexpected_ignoring_nones (inner) { return Some (unexpected) ; } cursor = rest ; } if cursor . eof () { None } else { Some ((cursor . span () , cursor . scope_delimiter ())) } }
    };
}

span_of_unexpected_ignoring_nones!();