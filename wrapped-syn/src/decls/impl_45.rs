macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl Token for Underscore { fn peek (cursor : Cursor) -> bool { if let Some ((ident , _rest)) = cursor . ident () { return ident == "_" ; } if let Some ((punct , _rest)) = cursor . punct () { return punct . as_char () == '_' ; } false } fn display () -> & 'static str { "`_`" } }
    };
}

impl_45!()