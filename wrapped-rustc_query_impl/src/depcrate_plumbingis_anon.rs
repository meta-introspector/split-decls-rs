// Generated macro for is_anon (macro)
macro_rules! Depcrate_plumbingis_anon {
() => {
// Module: crate::plumbing
// Provides: {"is_anon"}
// Dependencies: {}
macro_rules ! is_anon { ([]) => { { false } } ; ([(anon) $ ($ rest : tt) *]) => { { true } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { is_anon ! ([$ ($ modifiers) *]) } ; }
};
}
