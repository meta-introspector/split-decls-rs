// Generated macro for impl_1763 (impl)
macro_rules! Depcrate_tls13_key_scheduleimpl_1763 {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"impl_1763"}
// Dependencies: {}
impl KeyScheduleClientBeforeFinished { pub (crate) fn into_traffic (self , common : & mut CommonState , hs_hash : hash :: Output ,) -> (KeyScheduleTraffic , KeyScheduleExporter , KeyScheduleResumption ,) { let next = self . 0 ; debug_assert_eq ! (common . side , Side :: Client) ; let (client_secret , server_secret) = (& next . current_client_traffic_secret , & next . current_server_traffic_secret ,) ; next . ks . set_decrypter (server_secret , common) ; next . ks . set_encrypter (client_secret , common) ; if common . is_quic () { common . quic . traffic_secrets = Some (quic :: Secrets :: new (client_secret . clone () , server_secret . clone () , next . ks . suite , next . ks . suite . quic . unwrap () , common . side , common . quic . version ,)) ; } next . into_traffic (hs_hash) } }
};
}
