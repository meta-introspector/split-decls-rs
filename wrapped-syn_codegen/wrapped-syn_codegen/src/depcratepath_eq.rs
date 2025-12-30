// Generated macro for path_eq (function)
macro_rules! Depcratepath_eq {
() => {
// Module: crate
// Provides: {"path_eq"}
// Dependencies: {}
fn path_eq (a : & syn :: Path , b : & syn :: Path) -> bool { if a . global () != b . global () || a . segments . len () != b . segments . len () { return false ; } a . segments . iter () . zip (b . segments . iter ()) . all (| (a , b) | a . item () . ident . as_ref () == b . item () . ident . as_ref ()) }
};
}
