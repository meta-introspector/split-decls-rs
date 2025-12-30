// Generated macro for impl_1155 (impl)
macro_rules! Depcrate_segmenter_lstmimpl_1155 {
() => {
// Module: crate::segmenter::lstm
// Provides: {"impl_1155"}
// Dependencies: {}
impl IterableDataProviderCached < SegmenterLstmAutoV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { const SUPPORTED : [& DataMarkerAttributes ; 4] = [DataMarkerAttributes :: from_str_or_panic ("Burmese_codepoints_exclusive_model4_heavy") , DataMarkerAttributes :: from_str_or_panic ("Khmer_codepoints_exclusive_model4_heavy") , DataMarkerAttributes :: from_str_or_panic ("Lao_codepoints_exclusive_model4_heavy") , DataMarkerAttributes :: from_str_or_panic ("Thai_codepoints_exclusive_model4_heavy") ,] ; Ok (SUPPORTED . into_iter () . map (DataIdentifierCow :: from_marker_attributes) . collect ()) } }
};
}
