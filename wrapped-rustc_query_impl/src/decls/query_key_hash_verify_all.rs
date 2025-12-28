macro_rules! query_key_hash_verify_all {
    () => {
        pub fn query_key_hash_verify_all < 'tcx > (tcx : TyCtxt < 'tcx >) { if tcx . sess () . opts . unstable_opts . incremental_verify_ich || cfg ! (debug_assertions) { tcx . sess . time ("query_key_hash_verify_all" , | | { for verify in super :: QUERY_KEY_HASH_VERIFY . iter () { verify (tcx) ; } }) } }
    };
}

query_key_hash_verify_all!()