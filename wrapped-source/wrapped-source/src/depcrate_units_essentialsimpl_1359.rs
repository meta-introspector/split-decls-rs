// Generated macro for impl_1359 (impl)
macro_rules! Depcrate_units_essentialsimpl_1359 {
() => {
// Module: crate::units::essentials
// Provides: {"impl_1359"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < UnitsEssentialsV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { let units = self . cldr () ? . units () ; let locales = units . list_locales () ? ; Ok (locales . into_iter () . flat_map (| locale | { [DataMarkerAttributes :: from_str_or_panic ("long") , DataMarkerAttributes :: from_str_or_panic ("short") , DataMarkerAttributes :: from_str_or_panic ("narrow") ,] . into_iter () . map (move | length | DataIdentifierCow :: from_borrowed_and_owned (length , locale)) }) . collect ()) } }
};
}
