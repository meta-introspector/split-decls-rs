// Generated macro for KeyScheduleBeforeFinished (struct)
macro_rules! Depcrate_tls13_key_scheduleKeyScheduleBeforeFinished {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"KeyScheduleBeforeFinished"}
// Dependencies: {}
# [doc = " Keys derived (but not installed) before client's Finished message."] pub (crate) struct KeyScheduleBeforeFinished { ks : KeySchedule , current_client_traffic_secret : OkmBlock , current_server_traffic_secret : OkmBlock , current_exporter_secret : OkmBlock , }
};
}
