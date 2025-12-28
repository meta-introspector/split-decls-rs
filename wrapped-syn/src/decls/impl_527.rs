macro_rules! deps {
    () => {
        Result!();
        ParseStream!();
        Unexpected!();
        TokenBuffer!();
        Parser!();
    };
}

macro_rules! impl_527 {
    () => {
        deps!();
        impl < F , T > Parser for F where F : FnOnce (ParseStream) -> Result < T > , { type Output = T ; fn parse2 (self , tokens : TokenStream) -> Result < T > { let buf = TokenBuffer :: new2 (tokens) ; let state = tokens_to_parse_buffer (& buf) ; let node = self (& state) ? ; state . check_unexpected () ? ; if let Some ((unexpected_span , delimiter)) = span_of_unexpected_ignoring_nones (state . cursor ()) { Err (err_unexpected_token (unexpected_span , delimiter)) } else { Ok (node) } } fn __parse_scoped (self , scope : Span , tokens : TokenStream) -> Result < Self :: Output > { let buf = TokenBuffer :: new2 (tokens) ; let cursor = buf . begin () ; let unexpected = Rc :: new (Cell :: new (Unexpected :: None)) ; let state = new_parse_buffer (scope , cursor , unexpected) ; let node = self (& state) ? ; state . check_unexpected () ? ; if let Some ((unexpected_span , delimiter)) = span_of_unexpected_ignoring_nones (state . cursor ()) { Err (err_unexpected_token (unexpected_span , delimiter)) } else { Ok (node) } } }
    };
}

impl_527!()