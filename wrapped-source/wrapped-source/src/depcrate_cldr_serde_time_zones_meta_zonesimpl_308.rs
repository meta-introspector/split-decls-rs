// Generated macro for impl_308 (impl)
macro_rules! Depcrate_cldr_serde_time_zones_meta_zonesimpl_308 {
() => {
// Module: crate::cldr_serde::time_zones::meta_zones
// Provides: {"impl_308"}
// Dependencies: {}
impl TimeZonePeriod { pub (crate) fn iter (& self) -> impl Iterator < Item = (String , & Vec < MetazoneForPeriod >) > + '_ { self . 0 . iter () . flat_map (| (key , zone) | match zone { ZonePeriod :: Region (periods) => vec ! [(key . to_string () , periods)] , ZonePeriod :: LocationOrSubRegion (place) => place . iter () . flat_map (move | (key2 , location_or_subregion) | match location_or_subregion { MetaLocationOrSubRegion :: Location (periods) => { vec ! [(format ! ("{key}/{key2}") , periods)] } MetaLocationOrSubRegion :: SubRegion (subregion) => subregion . iter () . flat_map (move | (key3 , periods) | { vec ! [(format ! ("{key}/{key2}/{key3}") , periods)] }) . collect :: < Vec < _ > > () , } ,) . collect :: < Vec < _ > > () , }) } }
};
}
