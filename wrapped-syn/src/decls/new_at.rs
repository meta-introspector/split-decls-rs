macro_rules! deps {
    () => {
        Cursor!();
        Error!();
    };
}

macro_rules! new_at {
    () => {
        deps!();
        # [cfg (feature = "parsing")] pub (crate) fn new_at < T : Display > (scope : Span , cursor : Cursor , message : T) -> Error { if cursor . eof () { Error :: new (scope , format ! ("unexpected end of input, {}" , message)) } else { let span = crate :: buffer :: open_span_of_group (cursor) ; Error :: new (span , message) } }
    };
}

new_at!()