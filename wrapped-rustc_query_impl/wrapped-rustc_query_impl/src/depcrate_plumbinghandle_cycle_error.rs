// Generated macro for handle_cycle_error (macro)
macro_rules! Depcrate_plumbinghandle_cycle_error {
() => {
// Module: crate::plumbing
// Provides: {"handle_cycle_error"}
// Dependencies: {}
macro_rules ! handle_cycle_error { ([]) => { { rustc_query_system :: HandleCycleError :: Error } } ; ([(fatal_cycle) $ ($ rest : tt) *]) => { { rustc_query_system :: HandleCycleError :: Fatal } } ; ([(cycle_stash) $ ($ rest : tt) *]) => { { rustc_query_system :: HandleCycleError :: Stash } } ; ([(cycle_delay_bug) $ ($ rest : tt) *]) => { { rustc_query_system :: HandleCycleError :: DelayBug } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { handle_cycle_error ! ([$ ($ modifiers) *]) } ; }
};
}
