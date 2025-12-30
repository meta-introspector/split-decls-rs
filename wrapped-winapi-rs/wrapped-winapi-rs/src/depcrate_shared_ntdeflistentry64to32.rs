// Generated macro for ListEntry64To32 (function)
macro_rules! Depcrate_shared_ntdefListEntry64To32 {
() => {
// Module: crate::shared::ntdef
// Provides: {"ListEntry64To32"}
// Dependencies: {}
# [inline] pub unsafe fn ListEntry64To32 (l64 : PLIST_ENTRY64 , l32 : PLIST_ENTRY32) { (* l32) . Flink = (* l64) . Flink as ULONG ; (* l32) . Blink = (* l64) . Blink as ULONG ; }
};
}
