// Generated macro for bug (macro)
macro_rules! Depcrate_internal_macrosbug {
() => {
// Module: crate::internal_macros
// Provides: {"bug"}
// Dependencies: {}
# [doc = " `unreachable!()`, but better."] # [cfg (any (feature = "formatting" , feature = "parsing"))] macro_rules ! bug { () => { compile_error ! ("provide an error message to help fix a possible bug") } ; ($ descr : literal $ ($ rest : tt) ?) => { panic ! (concat ! ("internal error: " , $ descr) $ ($ rest) ?) } }
};
}
