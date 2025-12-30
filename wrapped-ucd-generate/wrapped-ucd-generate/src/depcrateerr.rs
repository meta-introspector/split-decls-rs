// Generated macro for err (macro)
macro_rules! Depcrateerr {
() => {
// Module: crate
// Provides: {"err"}
// Dependencies: {}
macro_rules ! err { ($ ($ tt : tt) *) => { Err (crate :: error :: Error :: Other (format ! ($ ($ tt) *))) } }
};
}
