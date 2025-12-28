macro_rules! deps {
    () => {
        QueryCtxt!();
    };
}

macro_rules! query_key_hash_verify {
    () => {
        deps!();
        pub (crate) fn query_key_hash_verify < 'tcx > (query : impl QueryConfig < QueryCtxt < 'tcx > > , qcx : QueryCtxt < 'tcx > ,) { let _timer = qcx . profiler () . generic_activity_with_arg ("query_key_hash_verify_for" , query . name ()) ; let mut map = UnordMap :: default () ; let cache = query . query_cache (qcx) ; cache . iter (& mut | key , _ , _ | { let node = DepNode :: construct (qcx . tcx , query . dep_kind () , key) ; if let Some (other_key) = map . insert (node , * key) { bug ! ("query key:\n\
                `{:?}`\n\
                and key:\n\
                `{:?}`\n\
                mapped to the same dep node:\n\
                {:?}" , key , other_key , node) ; } }) ; }
    };
}

query_key_hash_verify!();