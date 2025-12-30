// Generated macro for WorkerRegistrar (struct)
macro_rules! Depcrate_actor_registrarWorkerRegistrar {
() => {
// Module: crate::actor::registrar
// Provides: {"WorkerRegistrar"}
// Dependencies: {}
# [doc = " A Worker Registrar."] pub struct WorkerRegistrar < W , CODEC = Bincode > where W : Worker , CODEC : Codec , { _marker : PhantomData < (W , CODEC) > , }
};
}
