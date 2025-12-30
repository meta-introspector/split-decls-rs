// Generated macro for impl_151 (impl)
macro_rules! Depcrate_eventimpl_151 {
() => {
// Module: crate::event
// Provides: {"impl_151"}
// Dependencies: {}
impl Event { pub fn new (kind : EventKind) -> Self { Self { thread_id : thread :: current () . id () , kind , } } }
};
}
