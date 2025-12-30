// Generated macro for expand_if_cached (macro)
macro_rules! Depcrate_plumbingexpand_if_cached {
() => {
// Module: crate::plumbing
// Provides: {"expand_if_cached"}
// Dependencies: {}
macro_rules ! expand_if_cached { ([] , $ tokens : expr) => { { None } } ; ([(cache) $ ($ rest : tt) *] , $ tokens : expr) => { { Some ($ tokens) } } ; ([$ other : tt $ ($ modifiers : tt) *] , $ tokens : expr) => { expand_if_cached ! ([$ ($ modifiers) *] , $ tokens) } ; }
};
}
