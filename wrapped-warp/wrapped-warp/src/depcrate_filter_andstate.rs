// Generated macro for State (enum)
macro_rules! Depcrate_filter_andState {
() => {
// Module: crate::filter::and
// Provides: {"State"}
// Dependencies: {}
# [pin_project (project = StateProj)] enum State < T , TE , U : Filter > { First (# [pin] T , U) , Second (Option < TE > , # [pin] U :: Future) , Done , }
};
}
