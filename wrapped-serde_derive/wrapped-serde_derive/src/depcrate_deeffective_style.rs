// Generated macro for effective_style (function)
macro_rules! Depcrate_deeffective_style {
() => {
// Module: crate::de
// Provides: {"effective_style"}
// Dependencies: {}
fn effective_style (variant : & Variant) -> Style { match variant . style { Style :: Newtype if variant . fields [0] . attrs . skip_deserializing () => Style :: Unit , other => other , } }
};
}
