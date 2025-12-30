// Generated macro for test (module)
macro_rules! Depcrate_settings_quictest {
() => {
// Module: crate::settings::quic
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: QuicSettings ; use std :: time :: Duration ; # [test] fn timeouts_parse_as_milliseconds () { let quic = serde_json :: from_str :: < QuicSettings > (r#"{ "handshake_timeout_ms": 5000, "max_idle_timeout_ms": 7000 }"# ,) . unwrap () ; assert_eq ! (quic . handshake_timeout . unwrap () , Duration :: from_secs (5)) ; assert_eq ! (quic . max_idle_timeout . unwrap () , Duration :: from_secs (7)) ; } }
};
}
