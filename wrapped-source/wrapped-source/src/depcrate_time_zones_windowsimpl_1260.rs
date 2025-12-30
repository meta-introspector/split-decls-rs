// Generated macro for impl_1260 (impl)
macro_rules! Depcrate_time_zones_windowsimpl_1260 {
() => {
// Module: crate::time_zones::windows
// Provides: {"impl_1260"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < TimezoneIdentifiersWindowsV1 > for SourceDataProvider { fn iter_ids_cached (& self ,) -> Result < std :: collections :: HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (HashSet :: from_iter ([Default :: default ()])) } }
};
}
