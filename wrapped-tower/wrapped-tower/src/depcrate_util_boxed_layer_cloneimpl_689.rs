// Generated macro for impl_689 (impl)
macro_rules! Depcrate_util_boxed_layer_cloneimpl_689 {
() => {
// Module: crate::util::boxed::layer_clone
// Provides: {"impl_689"}
// Dependencies: {}
impl < In , T , U , E > Layer < In > for BoxCloneServiceLayer < In , T , U , E > { type Service = BoxCloneService < T , U , E > ; fn layer (& self , inner : In) -> Self :: Service { self . boxed . layer (inner) } }
};
}
