// Generated macro for impl_1500 (impl)
macro_rules! Depcrateimpl_1500 {
() => {
// Module: crate
// Provides: {"impl_1500"}
// Dependencies: {}
impl SourceDataProvider { fn check_req < M : DataMarker > (& self , req : DataRequest) -> Result < () , DataError > where SourceDataProvider : IterableDataProviderCached < M > , { if < M as DataMarker > :: INFO . is_singleton { if ! req . id . locale . is_unknown () { Err (DataErrorKind :: InvalidRequest) } else { Ok (()) } } else if ! self . populate_requests_cache () ? . contains (& req . id . as_cow ()) { Err (DataErrorKind :: IdentifierNotFound) } else { Ok (()) } . map_err (| e | e . with_req (< M as DataMarker > :: INFO , req)) } }
};
}
