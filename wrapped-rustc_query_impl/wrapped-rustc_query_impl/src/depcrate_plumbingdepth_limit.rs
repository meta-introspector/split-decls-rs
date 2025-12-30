// Generated macro for depth_limit (macro)
macro_rules! Depcrate_plumbingdepth_limit {
() => {
// Module: crate::plumbing
// Provides: {"depth_limit"}
// Dependencies: {}
macro_rules ! depth_limit { ([]) => { { false } } ; ([(depth_limit) $ ($ rest : tt) *]) => { { true } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { depth_limit ! ([$ ($ modifiers) *]) } ; }
};
}
