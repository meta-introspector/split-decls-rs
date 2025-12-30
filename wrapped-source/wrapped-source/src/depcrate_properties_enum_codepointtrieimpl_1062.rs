// Generated macro for impl_1062 (impl)
macro_rules! Depcrate_properties_enum_codepointtrieimpl_1062 {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"impl_1062"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < PropertyNameParseGeneralCategoryMaskV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { self . get_mask_prop ("gcm") ? ; Ok (HashSet :: from_iter ([Default :: default ()])) } }
};
}
