// Generated macro for bug (macro)
macro_rules! Depcratebug {
() => {
// Module: crate
// Provides: {"bug"}
// Dependencies: {}
# [allow (unused_macros , reason = "may not be used for all feature flag combinations")] macro_rules ! bug { () => { compile_error ! ("provide an error message to help fix a possible bug") } ; ($ descr : literal $ ($ rest : tt) ?) => { unreachable ! (concat ! ("internal error: " , $ descr) $ ($ rest) ?) } }
};
}
