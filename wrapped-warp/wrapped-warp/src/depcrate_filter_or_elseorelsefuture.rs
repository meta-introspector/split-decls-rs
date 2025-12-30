// Generated macro for OrElseFuture (struct)
macro_rules! Depcrate_filter_or_elseOrElseFuture {
() => {
// Module: crate::filter::or_else
// Provides: {"OrElseFuture"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [pin_project] pub struct OrElseFuture < T : Filter , F > where T : Filter , F : Func < T :: Error > , F :: Output : TryFuture < Ok = T :: Extract > + Send , { # [pin] state : State < T , F > , original_path_index : PathIndex , }
};
}
