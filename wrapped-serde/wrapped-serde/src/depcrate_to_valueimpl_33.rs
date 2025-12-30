// Generated macro for impl_33 (impl)
macro_rules! Depcrate_to_valueimpl_33 {
() => {
// Module: crate::to_value
// Provides: {"impl_33"}
// Dependencies: {}
impl < V : serde_core :: Serialize + ? Sized > ToValue < V > { # [doc = "\n    Adapt a reference to a [`serde_core::Serialize`] into an [`sval::Value`].\n    "] pub const fn new_borrowed < 'a > (value : & 'a V) -> & 'a ToValue < V > { unsafe { & * (value as * const _ as * const ToValue < V >) } } }
};
}
