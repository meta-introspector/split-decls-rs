// Generated macro for AndFuture (struct)
macro_rules! Depcrate_filter_andAndFuture {
() => {
// Module: crate::filter::and
// Provides: {"AndFuture"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [pin_project] pub struct AndFuture < T : Filter , U : Filter > { # [pin] state : State < T :: Future , T :: Extract , U > , }
};
}
