// Generated macro for impl_30 (impl)
macro_rules! Depcrate_iterator_backendimpl_30 {
() => {
// Module: crate::iterator::backend
// Provides: {"impl_30"}
// Dependencies: {}
impl DeliveryState { fn new () -> Self { let ids = (0 .. MAX_SIGNUM) . map (| _ | None) . collect () ; Self { closed : AtomicBool :: new (false) , registered_signal_ids : Mutex :: new (ids) , } } }
};
}
