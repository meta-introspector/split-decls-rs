// Generated macro for impl_699 (impl)
macro_rules! Depcrate_util_boxed_layer_clone_syncimpl_699 {
() => {
// Module: crate::util::boxed::layer_clone_sync
// Provides: {"impl_699"}
// Dependencies: {}
impl < In , T , U , E > Layer < In > for BoxCloneSyncServiceLayer < In , T , U , E > { type Service = BoxCloneSyncService < T , U , E > ; fn layer (& self , inner : In) -> Self :: Service { self . boxed . layer (inner) } }
};
}
