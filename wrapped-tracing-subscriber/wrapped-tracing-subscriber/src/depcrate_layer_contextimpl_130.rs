// Generated macro for impl_130 (impl)
macro_rules! Depcrate_layer_contextimpl_130 {
() => {
// Module: crate::layer::context
// Provides: {"impl_130"}
// Dependencies: {}
impl < S > Clone for Context < '_ , S > { # [inline] fn clone (& self) -> Self { let subscriber = self . subscriber . as_ref () . copied () ; Context { subscriber , # [cfg (all (feature = "registry" , feature = "std"))] filter : self . filter , } } }
};
}
