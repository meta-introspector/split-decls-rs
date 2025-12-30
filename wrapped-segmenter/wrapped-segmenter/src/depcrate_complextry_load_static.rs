// Generated macro for try_load_static (function)
macro_rules! Depcrate_complextry_load_static {
() => {
// Module: crate::complex
// Provides: {"try_load_static"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] fn try_load_static < M : DataMarker , P : DataProvider < M > + ? Sized > (provider : & P , model : & 'static DataMarkerAttributes ,) -> Result < Option < & 'static < M :: DataStruct as yoke :: Yokeable < 'static > > :: Output > , DataError > { provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes (model) , metadata : { let mut m = DataRequestMetadata :: default () ; m . silent = true ; m . attributes_prefix_match = true ; m } , }) . allow_identifier_not_found () . map (| r | r . and_then (| r | r . payload . get_static ())) }
};
}
