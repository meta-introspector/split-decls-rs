// Generated macro for format_tendril (macro)
macro_rules! Depcrate_tendrilformat_tendril {
() => {
// Module: crate::tendril
// Provides: {"format_tendril"}
// Dependencies: {}
# [doc = " Create a `StrTendril` through string formatting."] # [doc = ""] # [doc = " Works just like the standard `format!` macro."] # [macro_export] macro_rules ! format_tendril { ($ ($ arg : tt) *) => ($ crate :: StrTendril :: format (format_args ! ($ ($ arg) *))) }
};
}
