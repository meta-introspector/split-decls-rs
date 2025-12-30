// Generated macro for KeySchedulePreHandshake (struct)
macro_rules! Depcrate_tls13_key_scheduleKeySchedulePreHandshake {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"KeySchedulePreHandshake"}
// Dependencies: {}
# [doc = " The \"early secret\" stage of the key schedule."] # [doc = ""] # [doc = " Call [`KeySchedulePreHandshake::new`] to create it without"] # [doc = " a PSK, or use [`From<KeyScheduleEarly>`] to create it with"] # [doc = " a PSK."] # [doc = ""] # [doc = " ```text"] # [doc = "          0"] # [doc = "          |"] # [doc = "          v"] # [doc = " PSK -> HKDF-Extract = Early Secret"] # [doc = "          |"] # [doc = "          +-----> Derive-Secret(., \"ext binder\" | \"res binder\", \"\")"] # [doc = "          |                     = binder_key"] # [doc = "          |"] # [doc = "          +-----> Derive-Secret(., \"c e traffic\", ClientHello)"] # [doc = "          |                     = client_early_traffic_secret"] # [doc = "          |"] # [doc = "          +-----> Derive-Secret(., \"e exp master\", ClientHello)"] # [doc = "          |                     = early_exporter_master_secret"] # [doc = "          v"] # [doc = "    Derive-Secret(., \"derived\", \"\")"] # [doc = " ```"] pub (crate) struct KeySchedulePreHandshake { ks : KeySchedule , }
};
}
