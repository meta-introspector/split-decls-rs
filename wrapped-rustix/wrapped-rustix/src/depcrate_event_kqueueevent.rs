// Generated macro for Event (struct)
macro_rules! Depcrate_event_kqueueEvent {
() => {
// Module: crate::event::kqueue
// Provides: {"Event"}
// Dependencies: {}
# [doc = " A `kqueue` event for use with [`kevent`]."] # [repr (transparent)] # [derive (Copy , Clone)] pub struct Event { inner : kevent_t , }
};
}
