// Generated macro for impl_624 (impl)
macro_rules! Depcrate_common_stateimpl_624 {
() => {
// Module: crate::common_state
// Provides: {"impl_624"}
// Dependencies: {}
impl TemperCounters { fn received_warning_alert (& mut self) -> Result < () , Error > { match self . allowed_warning_alerts { 0 => Err (PeerMisbehaved :: TooManyWarningAlertsReceived . into ()) , _ => { self . allowed_warning_alerts -= 1 ; Ok (()) } } } fn received_renegotiation_request (& mut self) -> Result < () , Error > { match self . allowed_renegotiation_requests { 0 => Err (PeerMisbehaved :: TooManyRenegotiationRequests . into ()) , _ => { self . allowed_renegotiation_requests -= 1 ; Ok (()) } } } fn received_key_update_request (& mut self) -> Result < () , Error > { match self . allowed_key_update_requests { 0 => Err (PeerMisbehaved :: TooManyKeyUpdateRequests . into ()) , _ => { self . allowed_key_update_requests -= 1 ; Ok (()) } } } fn received_tls13_change_cipher_spec (& mut self) -> Result < () , Error > { match self . allowed_middlebox_ccs { 0 => Err (PeerMisbehaved :: IllegalMiddleboxChangeCipherSpec . into ()) , _ => { self . allowed_middlebox_ccs -= 1 ; Ok (()) } } } fn received_app_data (& mut self) { self . allowed_key_update_requests = Self :: INITIAL_KEY_UPDATE_REQUESTS ; } const INITIAL_KEY_UPDATE_REQUESTS : u8 = 32 ; }
};
}
