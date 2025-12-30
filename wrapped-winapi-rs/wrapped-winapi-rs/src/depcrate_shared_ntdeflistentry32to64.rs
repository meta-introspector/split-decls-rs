// Generated macro for ListEntry32To64 (function)
macro_rules! Depcrate_shared_ntdefListEntry32To64 {
() => {
// Module: crate::shared::ntdef
// Provides: {"ListEntry32To64"}
// Dependencies: {}
# [inline] pub unsafe fn ListEntry32To64 (l32 : PLIST_ENTRY32 , l64 : PLIST_ENTRY64) { (* l64) . Flink = (* l32) . Flink as ULONGLONG ; (* l64) . Blink = (* l32) . Blink as ULONGLONG ; }
};
}
