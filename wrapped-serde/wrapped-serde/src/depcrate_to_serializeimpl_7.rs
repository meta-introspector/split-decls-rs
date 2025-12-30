// Generated macro for impl_7 (impl)
macro_rules! Depcrate_to_serializeimpl_7 {
() => {
// Module: crate::to_serialize
// Provides: {"impl_7"}
// Dependencies: {}
impl < V : sval :: Value + ? Sized > ToSerialize < V > { # [doc = "\n    Adapt a reference to an [`sval::Value`] into a [`serde_core::Serialize`].\n    "] pub const fn new_borrowed < 'a > (value : & 'a V) -> & 'a ToSerialize < V > { unsafe { & * (value as * const _ as * const ToSerialize < V >) } } }
};
}
