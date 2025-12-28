macro_rules! deps {
    () => {
        Cursor!();
        Group!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl Token for Group { fn peek (cursor : Cursor) -> bool { cursor . group (Delimiter :: None) . is_some () } fn display () -> & 'static str { "invisible group" } }
    };
}

impl_61!()