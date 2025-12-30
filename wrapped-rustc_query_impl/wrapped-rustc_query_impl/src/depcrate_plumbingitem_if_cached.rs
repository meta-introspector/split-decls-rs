// Generated macro for item_if_cached (macro)
macro_rules! Depcrate_plumbingitem_if_cached {
() => {
// Module: crate::plumbing
// Provides: {"item_if_cached"}
// Dependencies: {}
macro_rules ! item_if_cached { ([] $ tokens : tt) => { } ; ([(cache) $ ($ rest : tt) *] { $ ($ tokens : tt) * }) => { $ ($ tokens) * } ; ([$ other : tt $ ($ modifiers : tt) *] $ tokens : tt) => { item_if_cached ! { [$ ($ modifiers) *] $ tokens } } ; }
};
}
