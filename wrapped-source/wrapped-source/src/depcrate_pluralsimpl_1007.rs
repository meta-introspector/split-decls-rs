// Generated macro for impl_1007 (impl)
macro_rules! Depcrate_pluralsimpl_1007 {
() => {
// Module: crate::plurals
// Provides: {"impl_1007"}
// Dependencies: {}
# [cfg (feature = "experimental")] impl IterableDataProviderCached < PluralsRangesV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . get_plural_ranges () ? . 0 . keys () . map (| l | DataIdentifierCow :: from_locale (DataLocale :: from (l))) . chain ([Default :: default ()]) . collect ()) } }
};
}
