// Generated macro for State (enum)
macro_rules! Depcrate_filter_or_elseState {
() => {
// Module: crate::filter::or_else
// Provides: {"State"}
// Dependencies: {}
# [pin_project (project = StateProj)] enum State < T , F > where T : Filter , F : Func < T :: Error > , F :: Output : TryFuture < Ok = T :: Extract > + Send , { First (# [pin] T :: Future , F) , Second (# [pin] F :: Output) , Done , }
};
}
