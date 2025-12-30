// Generated macro for impl_1754 (impl)
macro_rules! Depcrate_tls13_key_scheduleimpl_1754 {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"impl_1754"}
// Dependencies: {}
impl KeySchedulePreHandshake { # [doc = " Creates a key schedule without a PSK."] pub (crate) fn new (suite : & 'static Tls13CipherSuite) -> Self { Self { ks : KeySchedule :: new_with_empty_secret (suite) , } } # [doc = " `shared_secret` is the \"(EC)DHE\" secret input to"] # [doc = " \"HKDF-Extract\":"] # [doc = ""] # [doc = " ```text"] # [doc = " (EC)DHE -> HKDF-Extract = Handshake Secret"] # [doc = " ```"] pub (crate) fn into_handshake (mut self , shared_secret : SharedSecret ,) -> KeyScheduleHandshakeStart { self . ks . input_secret (shared_secret . secret_bytes ()) ; KeyScheduleHandshakeStart { ks : self . ks } } }
};
}
