// Generated macro for impl_148 (impl)
macro_rules! Depcrate_event_portimpl_148 {
() => {
// Module: crate::event::port
// Provides: {"impl_148"}
// Dependencies: {}
impl Event { # [doc = " Get the events associated with this event."] pub fn events (& self) -> i32 { self . 0 . portev_events } # [doc = " Get the event source associated with this event."] pub fn object (& self) -> usize { self . 0 . portev_object } # [doc = " Get the userdata associated with this event."] pub fn userdata (& self) -> * mut ffi :: c_void { self . 0 . portev_user } }
};
}
