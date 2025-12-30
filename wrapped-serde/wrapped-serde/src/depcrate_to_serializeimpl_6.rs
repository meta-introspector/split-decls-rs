// Generated macro for impl_6 (impl)
macro_rules! Depcrate_to_serializeimpl_6 {
() => {
// Module: crate::to_serialize
// Provides: {"impl_6"}
// Dependencies: {}
impl < V : sval :: Value > ToSerialize < V > { # [doc = "\n    Adapt an [`sval::Value`] into a [`serde_core::Serialize`].\n    "] pub const fn new (value : V) -> ToSerialize < V > { ToSerialize (value) } }
};
}
