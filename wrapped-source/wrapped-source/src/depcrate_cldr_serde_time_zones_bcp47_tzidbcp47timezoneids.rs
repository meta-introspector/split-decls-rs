// Generated macro for Bcp47TimeZoneIds (struct)
macro_rules! Depcrate_cldr_serde_time_zones_bcp47_tzidBcp47TimeZoneIds {
() => {
// Module: crate::cldr_serde::time_zones::bcp47_tzid
// Provides: {"Bcp47TimeZoneIds"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct Bcp47TimeZoneIds { pub (crate) _alias : String , pub (crate) _description : String , # [serde (flatten)] pub (crate) values : BTreeMap < TimeZone , Bcp47TzidAliasData > , }
};
}
