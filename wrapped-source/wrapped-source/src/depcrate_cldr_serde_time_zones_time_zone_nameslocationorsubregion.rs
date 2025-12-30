// Generated macro for LocationOrSubRegion (enum)
macro_rules! Depcrate_cldr_serde_time_zones_time_zone_namesLocationOrSubRegion {
() => {
// Module: crate::cldr_serde::time_zones::time_zone_names
// Provides: {"LocationOrSubRegion"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone , Deserialize)] # [serde (untagged)] pub (crate) enum LocationOrSubRegion { Location (Location) , SubRegion (BTreeMap < String , Location >) , }
};
}
