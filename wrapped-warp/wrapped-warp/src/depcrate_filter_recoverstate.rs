// Generated macro for State (enum)
macro_rules! Depcrate_filter_recoverState {
() => {
// Module: crate::filter::recover
// Provides: {"State"}
// Dependencies: {}
# [pin_project (project = StateProj)] enum State < T , F > where T : Filter , F : Func < T :: Error > , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : IsReject , { First (# [pin] T :: Future , F) , Second (# [pin] F :: Output) , Done , }
};
}
