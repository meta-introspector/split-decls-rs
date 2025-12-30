// Generated macro for impl_1504 (impl)
macro_rules! Depcrateimpl_1504 {
() => {
// Module: crate
// Provides: {"impl_1504"}
// Dependencies: {}
impl < M : DataMarker > IterableDataProvider < M > for SourceDataProvider where SourceDataProvider : IterableDataProviderCached < M > , { fn iter_ids (& self) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { Ok (if < M as DataMarker > :: INFO . is_singleton { [Default :: default ()] . into_iter () . collect () } else { self . populate_requests_cache () ? . iter () . map (| id | id . as_borrowed () . as_cow ()) . collect () }) } }
};
}
