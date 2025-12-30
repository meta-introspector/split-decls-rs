// Generated macro for EventDescSetId (function)
macro_rules! Depcrate_shared_evntprovEventDescSetId {
() => {
// Module: crate::shared::evntprov
// Provides: {"EventDescSetId"}
// Dependencies: {}
# [inline] pub unsafe fn EventDescSetId (EventDescriptor : PEVENT_DESCRIPTOR , Id : USHORT) -> PEVENT_DESCRIPTOR { (* EventDescriptor) . Id = Id ; EventDescriptor }
};
}
