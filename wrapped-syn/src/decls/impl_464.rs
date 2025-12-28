macro_rules! deps {
    () => {
        End!();
        Cursor!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl CustomToken for End { fn peek (cursor : Cursor) -> bool { cursor . eof () } fn display () -> & 'static str { "`)`" } }
    };
}

impl_464!()