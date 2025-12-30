// Generated macro for impl_1503 (impl)
macro_rules! Depcrateimpl_1503 {
() => {
// Module: crate
// Provides: {"impl_1503"}
// Dependencies: {}
impl SourceDataProvider { fn populate_requests_cache < M : DataMarker > (& self ,) -> Result < & HashSet < DataIdentifierCow < '_ > > , DataError > where SourceDataProvider : IterableDataProviderCached < M > , { self . requests_cache . insert_with (M :: INFO , | | Box :: new (OnceLock :: new ())) . get_or_init (| | self . iter_ids_cached ()) . as_ref () . map_err (| & e | e) } }
};
}
