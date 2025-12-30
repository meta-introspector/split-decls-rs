// Generated macro for impl_451 (impl)
macro_rules! Depcrate_msgs_persistimpl_451 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_451"}
// Dependencies: {}
impl Retrieved < & Tls13ClientSessionValue > { pub (crate) fn obfuscated_ticket_age (& self) -> u32 { let age_secs = self . retrieved_at . as_secs () . saturating_sub (self . value . common . epoch) ; let age_millis = age_secs as u32 * 1000 ; age_millis . wrapping_add (self . value . age_add) } }
};
}
