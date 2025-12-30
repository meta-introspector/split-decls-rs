// Generated macro for impl_951 (impl)
macro_rules! Depcrate_lintsimpl_951 {
() => {
// Module: crate::lints
// Provides: {"impl_951"}
// Dependencies: {}
impl MismatchedLifetimeSyntaxesSuggestion { fn make_optional_alternative (& mut self) { use MismatchedLifetimeSyntaxesSuggestion :: * ; let optional_alternative = match self { Implicit { optional_alternative , .. } | Mixed { optional_alternative , .. } | Explicit { optional_alternative , .. } => optional_alternative , } ; * optional_alternative = true ; } }
};
}
