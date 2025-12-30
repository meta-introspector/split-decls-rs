// Generated macro for KeyScheduleEarly (struct)
macro_rules! Depcrate_tls13_key_scheduleKeyScheduleEarly {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"KeyScheduleEarly"}
// Dependencies: {}
# [doc = " The \"early secret\" stage of the key schedule WITH a PSK."] # [doc = ""] # [doc = " This is only useful when you need to use one of the binder"] # [doc = " keys, the \"client_early_traffic_secret\", or"] # [doc = " \"early_exporter_master_secret\"."] # [doc = ""] # [doc = " See [`KeySchedulePreHandshake`] for more information."] pub (crate) struct KeyScheduleEarly { ks : KeySchedule , }
};
}
