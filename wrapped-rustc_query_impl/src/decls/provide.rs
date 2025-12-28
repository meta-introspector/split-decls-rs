macro_rules! provide {
    () => {
        pub fn provide (providers : & mut rustc_middle :: util :: Providers) { providers . hooks . alloc_self_profile_query_strings = alloc_self_profile_query_strings ; providers . hooks . query_key_hash_verify_all = query_key_hash_verify_all ; }
    };
}

provide!();