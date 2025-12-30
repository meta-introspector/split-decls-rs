// Generated macro for impl_325 (impl)
macro_rules! Depcrate_cldr_serde_time_zones_time_zone_namesimpl_325 {
() => {
// Module: crate::cldr_serde::time_zones::time_zone_names
// Provides: {"impl_325"}
// Dependencies: {}
impl Location { pub (crate) fn long_short (& self , long : bool) -> Option < & ZoneFormat > { if long { self . long . as_ref () } else { self . short . as_ref () } } }
};
}
