// Generated macro for impl_332 (impl)
macro_rules! Depcrate_cldr_serde_time_zones_time_zone_namesimpl_332 {
() => {
// Module: crate::cldr_serde::time_zones::time_zone_names
// Provides: {"impl_332"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for TimeZoneNames { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (TimeZoneNamesVisitor) } }
};
}
