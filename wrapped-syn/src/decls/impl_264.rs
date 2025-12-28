macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl CustomToken for private :: IdentAny { fn peek (cursor : Cursor) -> bool { cursor . ident () . is_some () } fn display () -> & 'static str { "identifier" } }
    };
}

impl_264!();