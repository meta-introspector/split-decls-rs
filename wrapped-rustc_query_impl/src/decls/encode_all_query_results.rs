macro_rules! encode_all_query_results {
    () => {
        pub (super) fn encode_all_query_results < 'tcx > (tcx : TyCtxt < 'tcx > , encoder : & mut CacheEncoder < '_ , 'tcx > , query_result_index : & mut EncodedDepNodeIndex ,) { for encode in super :: ENCODE_QUERY_RESULTS . iter () . copied () . flatten () { encode (tcx , encoder , query_result_index) ; } }
    };
}

encode_all_query_results!();