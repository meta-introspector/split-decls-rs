macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl Token for Bracket { fn peek (cursor : Cursor) -> bool { cursor . group (Delimiter :: Bracket) . is_some () } fn display () -> & 'static str { "square brackets" } }
    };
}

impl_60!()