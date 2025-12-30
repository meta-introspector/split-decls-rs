// Generated macro for macro_340 (macro)
macro_rules! Depcrate_load_constantmacro_340 {
() => {
// Module: crate::load::constant
// Provides: {"macro_340"}
// Dependencies: {}
pin_project ! { # [derive (Debug)] # [doc = " Wraps a type so that it implements [`Load`] and returns a constant load metric."] # [doc = ""] # [doc = " This load estimator is primarily useful for testing."] pub struct Constant < T , M > { inner : T , load : M , } }
};
}
