// Generated macro for impl_690 (impl)
macro_rules! Depcrate_util_boxed_layer_cloneimpl_690 {
() => {
// Module: crate::util::boxed::layer_clone
// Provides: {"impl_690"}
// Dependencies: {}
impl < In , T , U , E > Clone for BoxCloneServiceLayer < In , T , U , E > { fn clone (& self) -> Self { Self { boxed : Arc :: clone (& self . boxed) , } } }
};
}
