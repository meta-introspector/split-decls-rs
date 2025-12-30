// Generated macro for is_eval_always (macro)
macro_rules! Depcrate_plumbingis_eval_always {
() => {
// Module: crate::plumbing
// Provides: {"is_eval_always"}
// Dependencies: {}
macro_rules ! is_eval_always { ([]) => { { false } } ; ([(eval_always) $ ($ rest : tt) *]) => { { true } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { is_eval_always ! ([$ ($ modifiers) *]) } ; }
};
}
