macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [cfg (feature = "parsing")] impl < T : CustomToken > Token for T { fn peek (cursor : Cursor) -> bool { < Self as CustomToken > :: peek (cursor) } fn display () -> & 'static str { < Self as CustomToken > :: display () } }
    };
}

impl_36!();