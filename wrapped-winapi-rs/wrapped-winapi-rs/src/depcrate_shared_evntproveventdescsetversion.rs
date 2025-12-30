// Generated macro for EventDescSetVersion (function)
macro_rules! Depcrate_shared_evntprovEventDescSetVersion {
() => {
// Module: crate::shared::evntprov
// Provides: {"EventDescSetVersion"}
// Dependencies: {}
# [inline] pub unsafe fn EventDescSetVersion (EventDescriptor : PEVENT_DESCRIPTOR , Version : UCHAR ,) -> PEVENT_DESCRIPTOR { (* EventDescriptor) . Version = Version ; EventDescriptor }
};
}
