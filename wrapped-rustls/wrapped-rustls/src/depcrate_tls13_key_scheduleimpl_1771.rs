// Generated macro for impl_1771 (impl)
macro_rules! Depcrate_tls13_key_scheduleimpl_1771 {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"impl_1771"}
// Dependencies: {}
impl KeyScheduleResumption { pub (crate) fn derive_ticket_psk (& self , nonce : & [u8]) -> OkmBlock { self . ks . derive_ticket_psk (& self . resumption_master_secret , nonce) } }
};
}
