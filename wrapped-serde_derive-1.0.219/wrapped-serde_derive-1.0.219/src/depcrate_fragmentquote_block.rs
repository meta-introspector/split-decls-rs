// Generated macro for quote_block (macro)
macro_rules! Depcrate_fragmentquote_block {
() => {
// Module: crate::fragment
// Provides: {"quote_block"}
// Dependencies: {}
macro_rules ! quote_block { ($ ($ tt : tt) *) => { $ crate :: fragment :: Fragment :: Block (quote ! ($ ($ tt) *)) } }
};
}
