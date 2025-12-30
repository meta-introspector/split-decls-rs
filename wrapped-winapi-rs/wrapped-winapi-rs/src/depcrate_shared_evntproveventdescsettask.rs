// Generated macro for EventDescSetTask (function)
macro_rules! Depcrate_shared_evntprovEventDescSetTask {
() => {
// Module: crate::shared::evntprov
// Provides: {"EventDescSetTask"}
// Dependencies: {}
# [inline] pub unsafe fn EventDescSetTask (EventDescriptor : PEVENT_DESCRIPTOR , Task : USHORT ,) -> PEVENT_DESCRIPTOR { (* EventDescriptor) . Task = Task ; EventDescriptor }
};
}
