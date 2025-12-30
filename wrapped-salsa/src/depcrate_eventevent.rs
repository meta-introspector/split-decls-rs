// Generated macro for Event (struct)
macro_rules! Depcrate_eventEvent {
() => {
// Module: crate::event
// Provides: {"Event"}
// Dependencies: {}
# [doc = " The `Event` struct identifies various notable things that can"] # [doc = " occur during salsa execution. Instances of this struct are given"] # [doc = " to `salsa_event`."] # [derive (Debug)] pub struct Event { # [doc = " The id of the thread that triggered the event."] pub thread_id : ThreadId , # [doc = " What sort of event was it."] pub kind : EventKind , }
};
}
