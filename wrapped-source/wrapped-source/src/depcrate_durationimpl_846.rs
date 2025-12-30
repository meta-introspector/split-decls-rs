// Generated macro for impl_846 (impl)
macro_rules! Depcrate_durationimpl_846 {
() => {
// Module: crate::duration
// Provides: {"impl_846"}
// Dependencies: {}
# [cfg (feature = "experimental")] impl crate :: IterableDataProviderCached < DigitalDurationDataV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . numbers () . list_locales () ? . filter (| locale | { self . cldr () . unwrap () . units () . read_and_parse :: < cldr_serde :: units :: data :: Resource > (locale , "units.json") . is_ok () }) . map (DataIdentifierCow :: from_locale) . collect ()) } }
};
}
