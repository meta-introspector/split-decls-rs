// Generated macro for try_load (function)
macro_rules! Depcrate_complextry_load {
() => {
// Module: crate::complex
// Provides: {"try_load"}
// Dependencies: {}
fn try_load < M : DataMarker , P : DataProvider < M > + ? Sized > (provider : & P , model : & 'static DataMarkerAttributes ,) -> Result < Option < DataPayload < M > > , DataError > { provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes (model) , metadata : { let mut m = DataRequestMetadata :: default () ; m . silent = true ; m . attributes_prefix_match = true ; m } , }) . allow_identifier_not_found () . map (| r | r . map (| r | r . payload)) }
};
}
