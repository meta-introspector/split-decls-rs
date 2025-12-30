// Generated macro for impl_604 (impl)
macro_rules! Depcrate_spawn_ready_serviceimpl_604 {
() => {
// Module: crate::spawn_ready::service
// Provides: {"impl_604"}
// Dependencies: {}
impl < S > SpawnReady < S > { # [doc = " Creates a new [`SpawnReady`] wrapping `service`."] pub const fn new (service : S) -> Self { Self { inner : Inner :: Service (Some (service)) , } } # [doc = " Creates a layer that wraps services with [`SpawnReady`]."] pub fn layer () -> SpawnReadyLayer { SpawnReadyLayer :: default () } }
};
}
