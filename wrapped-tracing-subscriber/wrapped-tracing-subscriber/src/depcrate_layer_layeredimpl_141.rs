// Generated macro for impl_141 (impl)
macro_rules! Depcrate_layer_layeredimpl_141 {
() => {
// Module: crate::layer::layered
// Provides: {"impl_141"}
// Dependencies: {}
impl < L , S > Layered < L , S > where S : Subscriber , { fn ctx (& self) -> Context < '_ , S > { Context :: new (& self . inner) } }
};
}
