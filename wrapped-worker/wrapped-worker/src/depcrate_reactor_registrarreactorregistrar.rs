// Generated macro for ReactorRegistrar (struct)
macro_rules! Depcrate_reactor_registrarReactorRegistrar {
() => {
// Module: crate::reactor::registrar
// Provides: {"ReactorRegistrar"}
// Dependencies: {}
# [doc = " A registrar for reactor workers."] pub struct ReactorRegistrar < R , CODEC = Bincode > where R : Reactor + 'static , CODEC : Codec + 'static , { inner : WorkerRegistrar < ReactorWorker < R > , CODEC > , }
};
}
