// Generated macro for State (enum)
macro_rules! Depcrate_filter_thenState {
() => {
// Module: crate::filter::then
// Provides: {"State"}
// Dependencies: {}
# [pin_project (project = StateProj)] enum State < T , F > where T : TryFuture , F : Func < T :: Ok > , F :: Output : Future + Send , { First (# [pin] T , F) , Second (# [pin] F :: Output) , Done , }
};
}
