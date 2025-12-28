macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl Token for Paren { fn peek (cursor : Cursor) -> bool { cursor . group (Delimiter :: Parenthesis) . is_some () } fn display () -> & 'static str { "parentheses" } }
    };
}

impl_58!()