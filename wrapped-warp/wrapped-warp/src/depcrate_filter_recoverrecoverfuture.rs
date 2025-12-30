// Generated macro for RecoverFuture (struct)
macro_rules! Depcrate_filter_recoverRecoverFuture {
() => {
// Module: crate::filter::recover
// Provides: {"RecoverFuture"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [pin_project] pub struct RecoverFuture < T : Filter , F > where T : Filter , F : Func < T :: Error > , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : IsReject , { # [pin] state : State < T , F > , original_path_index : PathIndex , }
};
}
