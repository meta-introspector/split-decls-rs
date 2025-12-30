// Generated macro for OneshotRegistrar (struct)
macro_rules! Depcrate_oneshot_registrarOneshotRegistrar {
() => {
// Module: crate::oneshot::registrar
// Provides: {"OneshotRegistrar"}
// Dependencies: {}
# [doc = " A registrar for oneshot workers."] pub struct OneshotRegistrar < T , CODEC = Bincode > where T : Oneshot + 'static , CODEC : Codec + 'static , { inner : WorkerRegistrar < OneshotWorker < T > , CODEC > , }
};
}
