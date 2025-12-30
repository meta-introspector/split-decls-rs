// Generated macro for impl_1064 (impl)
macro_rules! Depcrate_properties_enum_codepointtrieimpl_1064 {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"impl_1064"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < PropertyEnumIndicConjunctBreakV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { self . get_enumerated_prop ("InCB") ? ; Ok (HashSet :: from_iter ([Default :: default ()])) } }
};
}
