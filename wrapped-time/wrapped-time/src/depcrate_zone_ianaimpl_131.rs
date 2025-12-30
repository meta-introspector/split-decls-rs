// Generated macro for impl_131 (impl)
macro_rules! Depcrate_zone_ianaimpl_131 {
() => {
// Module: crate::zone::iana
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a > Iterator for TimeZoneAndCanonicalIter < 'a > { type Item = TimeZoneAndCanonical < 'a > ; fn next (& mut self) -> Option < Self :: Item > { let (time_zone , canonical) = self . 0 . next () ? ; Some (TimeZoneAndCanonical { time_zone , canonical , }) } }
};
}
