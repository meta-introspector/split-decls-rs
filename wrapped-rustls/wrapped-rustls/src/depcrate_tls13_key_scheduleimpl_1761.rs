// Generated macro for impl_1761 (impl)
macro_rules! Depcrate_tls13_key_scheduleimpl_1761 {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"impl_1761"}
// Dependencies: {}
impl KeyScheduleBeforeFinished { fn new (mut ks : KeySchedule , hs_hash : hash :: Output , key_log : & dyn KeyLog , client_random : & [u8 ; 32] ,) -> Self { ks . input_empty () ; let current_client_traffic_secret = ks . derive_logged_secret (SecretKind :: ClientApplicationTrafficSecret , hs_hash . as_ref () , key_log , client_random ,) ; let current_server_traffic_secret = ks . derive_logged_secret (SecretKind :: ServerApplicationTrafficSecret , hs_hash . as_ref () , key_log , client_random ,) ; let current_exporter_secret = ks . derive_logged_secret (SecretKind :: ExporterMasterSecret , hs_hash . as_ref () , key_log , client_random ,) ; Self { ks , current_client_traffic_secret , current_server_traffic_secret , current_exporter_secret , } } pub (crate) fn into_traffic (self , hs_hash : hash :: Output ,) -> (KeyScheduleTraffic , KeyScheduleExporter , KeyScheduleResumption ,) { let Self { ks , current_client_traffic_secret , current_server_traffic_secret , current_exporter_secret , } = self ; let resumption_master_secret = ks . derive (SecretKind :: ResumptionMasterSecret , hs_hash . as_ref ()) ; (KeyScheduleTraffic { ks : ks . inner , current_client_traffic_secret , current_server_traffic_secret , } , KeyScheduleExporter { ks : ks . inner , current_exporter_secret , } , KeyScheduleResumption { ks : ks . inner , resumption_master_secret , } ,) } }
};
}
