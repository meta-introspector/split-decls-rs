// Generated macro for KeyScheduleClientBeforeFinished (struct)
macro_rules! Depcrate_tls13_key_scheduleKeyScheduleClientBeforeFinished {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"KeyScheduleClientBeforeFinished"}
// Dependencies: {}
# [doc = " Client-side key schedule before the finished message is sent."] # [doc = ""] # [doc = " This differs from `KeyScheduleTrafficWithClientFinishedPending` because"] # [doc = " none of the final traffic secrets are installed yet.  After the finished"] # [doc = " message is sent, `into_traffic()` does that."] pub (crate) struct KeyScheduleClientBeforeFinished (KeyScheduleBeforeFinished) ;
};
}
