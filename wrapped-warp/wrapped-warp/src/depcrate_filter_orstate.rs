// Generated macro for State (enum)
macro_rules! Depcrate_filter_orState {
() => {
// Module: crate::filter::or
// Provides: {"State"}
// Dependencies: {}
# [pin_project (project = StateProj)] enum State < T : Filter , U : Filter > { First (# [pin] T :: Future , U) , Second (Option < T :: Error > , # [pin] U :: Future) , Done , }
};
}
