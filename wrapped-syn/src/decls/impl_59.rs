macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl Token for Brace { fn peek (cursor : Cursor) -> bool { cursor . group (Delimiter :: Brace) . is_some () } fn display () -> & 'static str { "curly braces" } }
    };
}

impl_59!()