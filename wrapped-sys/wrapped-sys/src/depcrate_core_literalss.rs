// Generated macro for s (macro)
macro_rules! Depcrate_core_literalss {
() => {
// Module: crate::core::literals
// Provides: {"s"}
// Dependencies: {}
# [doc = " A literal UTF-8 string with a trailing null terminator."] # [macro_export] macro_rules ! s { ($ s : literal) => { :: core :: concat ! ($ s , '\0') . as_ptr () } ; }
};
}
