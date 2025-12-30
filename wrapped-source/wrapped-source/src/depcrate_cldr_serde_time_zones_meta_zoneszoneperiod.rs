// Generated macro for ZonePeriod (enum)
macro_rules! Depcrate_cldr_serde_time_zones_meta_zonesZonePeriod {
() => {
// Module: crate::cldr_serde::time_zones::meta_zones
// Provides: {"ZonePeriod"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone , Deserialize)] # [serde (untagged)] pub (crate) enum ZonePeriod { Region (Vec < MetazoneForPeriod >) , LocationOrSubRegion (BTreeMap < String , MetaLocationOrSubRegion >) , }
};
}
