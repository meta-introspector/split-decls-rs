// Generated macro for unerase (function)
macro_rules! Depcrate_errorunerase {
() => {
// Module: crate::error
// Provides: {"unerase"}
// Dependencies: {}
pub (crate) fn unerase < E : serde :: de :: Error > (err : Error) -> E { err . as_serde () }
};
}
