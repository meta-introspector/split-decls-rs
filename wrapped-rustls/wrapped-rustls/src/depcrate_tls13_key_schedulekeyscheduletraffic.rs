// Generated macro for KeyScheduleTraffic (struct)
macro_rules! Depcrate_tls13_key_scheduleKeyScheduleTraffic {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"KeyScheduleTraffic"}
// Dependencies: {}
# [doc = " KeySchedule during traffic stage.  All traffic & exporter keys are guaranteed"] # [doc = " to be available."] pub (crate) struct KeyScheduleTraffic { ks : KeyScheduleSuite , current_client_traffic_secret : OkmBlock , current_server_traffic_secret : OkmBlock , }
};
}
