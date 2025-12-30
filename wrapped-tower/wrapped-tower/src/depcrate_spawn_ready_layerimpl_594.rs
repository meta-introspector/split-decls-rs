// Generated macro for impl_594 (impl)
macro_rules! Depcrate_spawn_ready_layerimpl_594 {
() => {
// Module: crate::spawn_ready::layer
// Provides: {"impl_594"}
// Dependencies: {}
impl < S > tower_layer :: Layer < S > for SpawnReadyLayer { type Service = super :: SpawnReady < S > ; fn layer (& self , service : S) -> Self :: Service { super :: SpawnReady :: new (service) } }
};
}
