// Generated macro for spawn_with (function)
macro_rules! Depcrate_mockspawn_with {
() => {
// Module: crate::mock
// Provides: {"spawn_with"}
// Dependencies: {}
# [doc = " Spawn a Service via the provided wrapper closure."] pub fn spawn_with < T , U , F , S > (f : F) -> (Spawn < S > , Handle < T , U >) where F : Fn (Mock < T , U >) -> S , { let (svc , handle) = pair () ; let svc = f (svc) ; (Spawn :: new (svc) , handle) }
};
}
