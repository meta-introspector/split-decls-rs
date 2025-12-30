// Generated macro for erase (function)
macro_rules! Depcrate_errorerase {
() => {
// Module: crate::error
// Provides: {"erase"}
// Dependencies: {}
pub (crate) fn erase < E : serde :: de :: Error > (err : E) -> Error { serde :: de :: Error :: custom (err) }
};
}
