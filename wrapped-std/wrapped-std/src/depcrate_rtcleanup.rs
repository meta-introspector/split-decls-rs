// Generated macro for cleanup (function)
macro_rules! Depcrate_rtcleanup {
() => {
// Module: crate::rt
// Provides: {"cleanup"}
// Dependencies: {}
pub (crate) fn cleanup () { static CLEANUP : Once = Once :: new () ; CLEANUP . call_once (| | unsafe { crate :: io :: cleanup () ; sys :: cleanup () ; }) ; }
};
}
