// Generated macro for MetaLocationOrSubRegion (enum)
macro_rules! Depcrate_cldr_serde_time_zones_meta_zonesMetaLocationOrSubRegion {
() => {
// Module: crate::cldr_serde::time_zones::meta_zones
// Provides: {"MetaLocationOrSubRegion"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone , Deserialize)] # [serde (untagged)] pub (crate) enum MetaLocationOrSubRegion { Location (Vec < MetazoneForPeriod >) , SubRegion (BTreeMap < String , Vec < MetazoneForPeriod > >) , }
};
}
