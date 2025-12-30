// Generated macro for impl_386 (impl)
macro_rules! Depcrate_metrics_tokio_taskimpl_386 {
() => {
// Module: crate::metrics::tokio_task
// Provides: {"impl_386"}
// Dependencies: {}
impl < F , M > Instrumented < F , M > where M : Metrics , { fn new (name : & str , metrics : M , future : F) -> Self { let name = Arc :: from (name) ; Self { future , name , metrics , timer : Arc :: new (Mutex :: new (Some (Instant :: now ()))) , } } }
};
}
