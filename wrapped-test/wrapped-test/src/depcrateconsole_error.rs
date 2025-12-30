// Generated macro for console_error (macro)
macro_rules! Depcrateconsole_error {
() => {
// Module: crate
// Provides: {"console_error"}
// Dependencies: {}
# [doc = " Helper macro which acts like `println!` only routes to `console.error`"] # [doc = " instead."] # [macro_export] macro_rules ! console_error { ($ ($ arg : tt) *) => ($ crate :: __rt :: console_error (& format_args ! ($ ($ arg) *))) }
};
}
