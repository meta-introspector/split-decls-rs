macro_rules! deps {
    () => {
        Lookahead1!();
        Cursor!();
    };
}

macro_rules! peek_impl {
    () => {
        deps!();
        fn peek_impl (lookahead : & Lookahead1 , peek : fn (Cursor) -> bool , display : fn () -> & 'static str ,) -> bool { if peek (lookahead . cursor) { return true ; } lookahead . comparisons . borrow_mut () . push (display ()) ; false }
    };
}

peek_impl!();