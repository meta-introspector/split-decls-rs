// Generated macro for ThenFuture (struct)
macro_rules! Depcrate_filter_thenThenFuture {
() => {
// Module: crate::filter::then
// Provides: {"ThenFuture"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [pin_project] pub struct ThenFuture < T , F > where T : Filter , F : Func < T :: Extract > , F :: Output : Future + Send , { # [pin] state : State < T :: Future , F > , }
};
}
