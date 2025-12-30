// Generated macro for impl_89 (impl)
macro_rules! Depcrate_cldr_serde_caimpl_89 {
() => {
// Module: crate::cldr_serde::ca
// Provides: {"impl_89"}
// Dependencies: {}
impl LengthPattern { # [doc = " Get the pattern, dropping the numbering system if present."] pub (crate) fn get_pattern (& self) -> & String { match self { Self :: Plain (pattern) => pattern , Self :: WithNumberingSystems { pattern , .. } => pattern , } } }
};
}
