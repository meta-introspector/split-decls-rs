// Generated macro for feedable (macro)
macro_rules! Depcrate_plumbingfeedable {
() => {
// Module: crate::plumbing
// Provides: {"feedable"}
// Dependencies: {}
macro_rules ! feedable { ([]) => { { false } } ; ([(feedable) $ ($ rest : tt) *]) => { { true } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { feedable ! ([$ ($ modifiers) *]) } ; }
};
}
