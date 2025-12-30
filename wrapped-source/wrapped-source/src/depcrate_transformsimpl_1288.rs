// Generated macro for impl_1288 (impl)
macro_rules! Depcrate_transformsimpl_1288 {
() => {
// Module: crate::transforms
// Provides: {"impl_1288"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < TransliteratorRulesV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { Ok (self . cldr () ? . transforms () ? . lock () . expect ("poison") . as_provider_unstable (self , self , self) ? . iter_ids () ? . into_iter () . map (| id | id . as_borrowed () . into_owned ()) . collect ()) } }
};
}
