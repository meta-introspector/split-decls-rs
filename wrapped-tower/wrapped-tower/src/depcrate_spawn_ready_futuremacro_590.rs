// Generated macro for macro_590 (macro)
macro_rules! Depcrate_spawn_ready_futuremacro_590 {
() => {
// Module: crate::spawn_ready::future
// Provides: {"macro_590"}
// Dependencies: {}
opaque_future ! { # [doc = " Response future from [`SpawnReady`] services."] # [doc = ""] # [doc = " [`SpawnReady`]: crate::spawn_ready::SpawnReady"] pub type ResponseFuture < F , E > = futures_util :: future :: MapErr < F , fn (E) -> crate :: BoxError >; }
};
}
