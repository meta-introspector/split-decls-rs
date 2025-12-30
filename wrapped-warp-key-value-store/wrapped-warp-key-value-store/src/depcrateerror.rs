// Generated macro for error (function)
macro_rules! Depcrateerror {
() => {
// Module: crate
// Provides: {"error"}
// Dependencies: {}
pub fn error () -> impl Filter < Extract = (& 'static str ,) , Error = Rejection > + Clone { warp :: get () . and (path ! ("debug" / "error")) . and_then (| | async move { Err (warp :: reject :: custom (InternalError)) }) }
};
}
