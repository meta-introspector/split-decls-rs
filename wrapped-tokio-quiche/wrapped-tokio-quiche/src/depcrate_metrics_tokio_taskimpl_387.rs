// Generated macro for impl_387 (impl)
macro_rules! Depcrate_metrics_tokio_taskimpl_387 {
() => {
// Module: crate::metrics::tokio_task
// Provides: {"impl_387"}
// Dependencies: {}
impl < F : Future , M : Metrics > Future for Instrumented < F , M > { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let total_timer = Instant :: now () ; let maybe_schedule_timer = self . timer . lock () . unwrap () . take () ; if let Some (schedule_timer) = maybe_schedule_timer { let elapsed = schedule_timer . elapsed () ; self . metrics . tokio_runtime_task_schedule_delay_histogram (& self . name) . observe (elapsed . as_nanos () as u64) ; } let projected = self . project () ; let waker = Waker :: from (Arc :: new (InstrumentedWaker { timer : Arc :: clone (projected . timer) , waker : cx . waker () . clone () , })) ; let mut new_cx = Context :: from_waker (& waker) ; let timer = Instant :: now () ; let output = projected . future . poll (& mut new_cx) ; let elapsed = timer . elapsed () ; projected . metrics . tokio_runtime_task_poll_duration_histogram (projected . name) . observe (elapsed . as_nanos () as u64) ; let total_elapsed = total_timer . elapsed () ; projected . metrics . tokio_runtime_task_total_poll_time_micros (projected . name) . inc_by (total_elapsed . as_micros () as u64) ; output } }
};
}
