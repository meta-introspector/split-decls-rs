// Generated macro for State (enum)
macro_rules! Depcrate_filter_and_thenState {
() => {
// Module: crate::filter::and_then
// Provides: {"State"}
// Dependencies: {}
# [pin_project (project = StateProj)] enum State < T , F > where T : TryFuture , F : Func < T :: Ok > , F :: Output : TryFuture + Send , < F :: Output as TryFuture > :: Error : CombineRejection < T :: Error > , { First (# [pin] T , F) , Second (# [pin] F :: Output) , Done , }
};
}
