// Generated macro for display (function)
macro_rules! Depcrate_fielddisplay {
() => {
// Module: crate::field
// Provides: {"display"}
// Dependencies: {}
# [doc = " Wraps a type implementing `fmt::Display` as a `Value` that can be"] # [doc = " recorded using its `Display` implementation."] pub fn display < T > (t : T) -> DisplayValue < T > where T : fmt :: Display , { DisplayValue (t) }
};
}
