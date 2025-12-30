// Generated macro for impl_471 (impl)
macro_rules! Depcrate_msgs_persistimpl_471 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_471"}
// Dependencies: {}
impl Tls13ServerSessionValue { pub (crate) fn new (common : CommonServerSessionValue , secret : & [u8] , age_obfuscation_offset : u32 ,) -> Self { Self { common , secret : Zeroizing :: new (PayloadU8 :: new (secret . to_vec ())) , age_obfuscation_offset , freshness : None , } } pub (crate) fn set_freshness (mut self , obfuscated_client_age_ms : u32 , time_now : UnixTime ,) -> Self { let client_age_ms = obfuscated_client_age_ms . wrapping_sub (self . age_obfuscation_offset) ; let server_age_ms = (time_now . as_secs () . saturating_sub (self . common . creation_time_sec) as u32) . saturating_mul (1000) ; let age_difference = server_age_ms . abs_diff (client_age_ms) ; self . freshness = Some (age_difference <= MAX_FRESHNESS_SKEW_MS) ; self } pub (crate) fn is_fresh (& self) -> bool { self . freshness . unwrap_or_default () } }
};
}
