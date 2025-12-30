// Generated macro for EventDescZero (function)
macro_rules! Depcrate_shared_evntprovEventDescZero {
() => {
// Module: crate::shared::evntprov
// Provides: {"EventDescZero"}
// Dependencies: {}
# [inline] pub unsafe fn EventDescZero (EventDescriptor : PEVENT_DESCRIPTOR) { use core :: ptr :: write_bytes ; write_bytes (EventDescriptor , 0 , 16) ; }
};
}
