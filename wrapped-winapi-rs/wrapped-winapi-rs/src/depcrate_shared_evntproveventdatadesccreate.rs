// Generated macro for EventDataDescCreate (function)
macro_rules! Depcrate_shared_evntprovEventDataDescCreate {
() => {
// Module: crate::shared::evntprov
// Provides: {"EventDataDescCreate"}
// Dependencies: {}
# [inline] pub unsafe fn EventDataDescCreate (EventDataDescriptor : PEVENT_DATA_DESCRIPTOR , DataPtr : * const VOID , DataSize : ULONG ,) { (* EventDataDescriptor) . Ptr = DataPtr as ULONGLONG ; (* EventDataDescriptor) . Size = DataSize ; * (* EventDataDescriptor) . u . Reserved_mut () = 0 ; }
};
}
