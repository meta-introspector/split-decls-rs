// Generated macro for macro_24 (macro)
macro_rules! Depcrate_backtracemacro_24 {
() => {
// Module: crate::backtrace
// Provides: {"macro_24"}
// Dependencies: {}
define_class ! (# [unsafe (super = NSObject)] struct Thrower ; impl Thrower { # [unsafe (method (backtrace))] fn __backtrace () -> * mut c_void { let backtrace = backtrace :: Backtrace :: new () ; Box :: into_raw (Box :: new (backtrace)) . cast () } }) ;
};
}
