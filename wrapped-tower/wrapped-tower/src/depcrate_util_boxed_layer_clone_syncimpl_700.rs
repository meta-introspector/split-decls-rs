// Generated macro for impl_700 (impl)
macro_rules! Depcrate_util_boxed_layer_clone_syncimpl_700 {
() => {
// Module: crate::util::boxed::layer_clone_sync
// Provides: {"impl_700"}
// Dependencies: {}
impl < In , T , U , E > Clone for BoxCloneSyncServiceLayer < In , T , U , E > { fn clone (& self) -> Self { Self { boxed : Arc :: clone (& self . boxed) , } } }
};
}
