// Generated macro for AndThenFuture (struct)
macro_rules! Depcrate_filter_and_thenAndThenFuture {
() => {
// Module: crate::filter::and_then
// Provides: {"AndThenFuture"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [pin_project] pub struct AndThenFuture < T , F > where T : Filter , F : Func < T :: Extract > , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : CombineRejection < T :: Error > , { # [pin] state : State < T :: Future , F > , }
};
}
