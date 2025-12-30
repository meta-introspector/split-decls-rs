// Generated macro for peek_impl (function)
macro_rules! Depcrate_lookaheadpeek_impl {
() => {
// Module: crate::lookahead
// Provides: {"peek_impl"}
// Dependencies: {}
fn peek_impl (lookahead : & Lookahead1 , peek : fn (Cursor) -> bool , display : fn () -> & 'static str ,) -> bool { if peek (lookahead . cursor) { return true ; } lookahead . comparisons . borrow_mut () . push (display ()) ; false }
};
}
