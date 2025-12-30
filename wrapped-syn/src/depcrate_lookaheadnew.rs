// Generated macro for new (function)
macro_rules! Depcrate_lookaheadnew {
() => {
// Module: crate::lookahead
// Provides: {"new"}
// Dependencies: {}
pub (crate) fn new (scope : Span , cursor : Cursor) -> Lookahead1 { Lookahead1 { scope , cursor , comparisons : RefCell :: new (Vec :: new ()) , } }
};
}
