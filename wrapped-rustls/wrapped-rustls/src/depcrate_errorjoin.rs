// Generated macro for join (function)
macro_rules! Depcrate_errorjoin {
() => {
// Module: crate::error
// Provides: {"join"}
// Dependencies: {}
fn join < T : fmt :: Debug > (items : & [T]) -> String { items . iter () . map (| x | format ! ("{x:?}")) . collect :: < Vec < String > > () . join (" or ") }
};
}
