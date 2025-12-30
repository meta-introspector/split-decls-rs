// Generated macro for impl_1407 (impl)
macro_rules! Depcrate_units_idsimpl_1407 {
() => {
// Module: crate::units::ids
// Provides: {"impl_1407"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < UnitIdsV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { let units_data : & cldr_serde :: units :: info :: Resource = self . cldr () ? . core () . read_and_parse ("supplemental/units.json") ? ; let ids_set = units_data . unit_ids_map () ? . keys () . map (| unit_name | { DataIdentifierCow :: from_marker_attributes_owned (DataMarkerAttributes :: try_from_string (unit_name . clone ()) . unwrap () ,) }) . collect () ; Ok (ids_set) } }
};
}
