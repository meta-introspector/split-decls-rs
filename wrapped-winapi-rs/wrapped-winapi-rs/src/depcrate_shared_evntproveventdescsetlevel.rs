// Generated macro for EventDescSetLevel (function)
macro_rules! Depcrate_shared_evntprovEventDescSetLevel {
() => {
// Module: crate::shared::evntprov
// Provides: {"EventDescSetLevel"}
// Dependencies: {}
# [inline] pub unsafe fn EventDescSetLevel (EventDescriptor : PEVENT_DESCRIPTOR , Level : UCHAR ,) -> PEVENT_DESCRIPTOR { (* EventDescriptor) . Level = Level ; EventDescriptor }
};
}
