// Generated macro for impl_32 (impl)
macro_rules! Depcrate_to_valueimpl_32 {
() => {
// Module: crate::to_value
// Provides: {"impl_32"}
// Dependencies: {}
impl < V : serde_core :: Serialize > ToValue < V > { # [doc = "\n    Adapt a [`serde_core::Serialize`] into a [`sval::Value`].\n    "] pub const fn new (value : V) -> ToValue < V > { ToValue (value) } }
};
}
