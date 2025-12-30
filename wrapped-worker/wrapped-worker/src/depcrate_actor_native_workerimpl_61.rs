// Generated macro for impl_61 (impl)
macro_rules! Depcrate_actor_native_workerimpl_61 {
() => {
// Module: crate::actor::native_worker
// Provides: {"impl_61"}
// Dependencies: {}
impl WorkerSelf for DedicatedWorker { type GlobalScope = DedicatedWorkerGlobalScope ; fn worker_self () -> Self :: GlobalScope { JsValue :: from (js_sys :: global ()) . into () } }
};
}
