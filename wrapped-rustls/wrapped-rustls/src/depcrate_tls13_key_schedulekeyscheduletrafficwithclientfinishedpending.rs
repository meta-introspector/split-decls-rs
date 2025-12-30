// Generated macro for KeyScheduleTrafficWithClientFinishedPending (struct)
macro_rules! Depcrate_tls13_key_scheduleKeyScheduleTrafficWithClientFinishedPending {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"KeyScheduleTrafficWithClientFinishedPending"}
// Dependencies: {}
# [doc = " KeySchedule during traffic stage, retaining the ability to calculate the client's"] # [doc = " finished verify_data. The traffic stage key schedule can be extracted from it"] # [doc = " through signing the client finished hash."] pub (crate) struct KeyScheduleTrafficWithClientFinishedPending { handshake_client_traffic_secret : OkmBlock , before_finished : KeyScheduleBeforeFinished , }
};
}
