// Generated macro for effective_style (function)
macro_rules! Depcrate_sereffective_style {
() => {
// Module: crate::ser
// Provides: {"effective_style"}
// Dependencies: {}
fn effective_style (variant : & Variant) -> Style { match variant . style { Style :: Newtype if variant . fields [0] . attrs . skip_serializing () => Style :: Unit , other => other , } }
};
}
