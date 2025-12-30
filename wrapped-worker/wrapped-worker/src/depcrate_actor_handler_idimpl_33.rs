// Generated macro for impl_33 (impl)
macro_rules! Depcrate_actor_handler_idimpl_33 {
() => {
// Module: crate::actor::handler_id
// Provides: {"impl_33"}
// Dependencies: {}
impl HandlerId { pub (crate) fn new () -> Self { static CTR : AtomicUsize = AtomicUsize :: new (0) ; let id = CTR . fetch_add (1 , Ordering :: SeqCst) ; HandlerId (id) } }
};
}
