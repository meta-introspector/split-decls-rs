macro_rules! push_hash64 {
    () => {
        fn push_hash64 (hash : u64 , output : & mut String) { let hash = v0 :: encode_integer_62 (hash) ; let hash_len = hash . len () ; let _ = write ! (output , "{hash_len}H{}" , & hash [.. hash_len - 1]) ; }
    };
}

push_hash64!()