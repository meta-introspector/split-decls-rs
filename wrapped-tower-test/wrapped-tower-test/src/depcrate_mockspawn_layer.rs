// Generated macro for spawn_layer (function)
macro_rules! Depcrate_mockspawn_layer {
() => {
// Module: crate::mock
// Provides: {"spawn_layer"}
// Dependencies: {}
# [doc = " Spawn a layer onto a mock service."] pub fn spawn_layer < T , U , L > (layer : L) -> (Spawn < L :: Service > , Handle < T , U >) where L : Layer < Mock < T , U > > , { let (inner , handle) = pair () ; let svc = layer . layer (inner) ; (Spawn :: new (svc) , handle) }
};
}
