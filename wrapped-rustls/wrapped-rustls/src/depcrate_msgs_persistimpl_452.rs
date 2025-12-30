// Generated macro for impl_452 (impl)
macro_rules! Depcrate_msgs_persistimpl_452 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_452"}
// Dependencies: {}
impl < T : core :: ops :: Deref < Target = ClientSessionCommon > > Retrieved < T > { pub (crate) fn has_expired (& self) -> bool { let common = & * self . value ; common . lifetime != Duration :: ZERO && common . epoch . saturating_add (common . lifetime . as_secs ()) < self . retrieved_at . as_secs () } }
};
}
