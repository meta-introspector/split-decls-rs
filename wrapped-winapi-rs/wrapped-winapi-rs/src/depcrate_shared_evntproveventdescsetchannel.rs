// Generated macro for EventDescSetChannel (function)
macro_rules! Depcrate_shared_evntprovEventDescSetChannel {
() => {
// Module: crate::shared::evntprov
// Provides: {"EventDescSetChannel"}
// Dependencies: {}
# [inline] pub unsafe fn EventDescSetChannel (EventDescriptor : PEVENT_DESCRIPTOR , Channel : UCHAR ,) -> PEVENT_DESCRIPTOR { (* EventDescriptor) . Channel = Channel ; EventDescriptor }
};
}
