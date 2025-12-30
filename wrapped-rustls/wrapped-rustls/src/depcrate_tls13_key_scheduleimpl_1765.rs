// Generated macro for impl_1765 (impl)
macro_rules! Depcrate_tls13_key_scheduleimpl_1765 {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"impl_1765"}
// Dependencies: {}
impl KeyScheduleTrafficWithClientFinishedPending { pub (crate) fn update_decrypter (& self , common : & mut CommonState) { debug_assert_eq ! (common . side , Side :: Server) ; self . before_finished . ks . set_decrypter (& self . handshake_client_traffic_secret , common) ; } pub (crate) fn sign_client_finish (self , hs_hash : & hash :: Output , common : & mut CommonState ,) -> (KeyScheduleBeforeFinished , hmac :: PublicTag) { debug_assert_eq ! (common . side , Side :: Server) ; let tag = self . before_finished . ks . sign_finish (& self . handshake_client_traffic_secret , hs_hash) ; self . before_finished . ks . set_decrypter (& self . before_finished . current_client_traffic_secret , common ,) ; (self . before_finished , tag) } }
};
}
