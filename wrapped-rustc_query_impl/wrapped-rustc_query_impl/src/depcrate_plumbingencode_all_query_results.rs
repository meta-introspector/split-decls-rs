// Generated macro for encode_all_query_results (function)
macro_rules! Depcrate_plumbingencode_all_query_results {
() => {
// Module: crate::plumbing
// Provides: {"encode_all_query_results"}
// Dependencies: {}
pub (super) fn encode_all_query_results < 'tcx > (tcx : TyCtxt < 'tcx > , encoder : & mut CacheEncoder < '_ , 'tcx > , query_result_index : & mut EncodedDepNodeIndex ,) { for encode in super :: ENCODE_QUERY_RESULTS . iter () . copied () . flatten () { encode (tcx , encoder , query_result_index) ; } }
};
}
