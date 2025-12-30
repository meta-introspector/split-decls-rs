// Generated macro for impl_679 (impl)
macro_rules! Depcrate_util_boxed_layerimpl_679 {
() => {
// Module: crate::util::boxed::layer
// Provides: {"impl_679"}
// Dependencies: {}
impl < In , T , U , E > Layer < In > for BoxLayer < In , T , U , E > { type Service = BoxService < T , U , E > ; fn layer (& self , inner : In) -> Self :: Service { self . boxed . layer (inner) } }
};
}
