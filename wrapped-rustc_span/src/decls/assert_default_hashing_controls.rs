macro_rules! deps {
    () => {
        ExpnData!();
        HashStableContext!();
    };
}

macro_rules! assert_default_hashing_controls {
    () => {
        deps!();
        # [doc = " Assert that the provided `HashStableContext` is configured with the 'default'"] # [doc = " `HashingControls`. We should always have bailed out before getting to here"] # [doc = " with a non-default mode. With this check in place, we can avoid the need"] # [doc = " to maintain separate versions of `ExpnData` hashes for each permutation"] # [doc = " of `HashingControls` settings."] fn assert_default_hashing_controls (ctx : & impl HashStableContext , msg : & str) { match ctx . hashing_controls () { HashingControls { hash_spans } if hash_spans != ctx . unstable_opts_incremental_ignore_spans () => { } other => panic ! ("Attempted hashing of {msg} with non-default HashingControls: {other:?}") , } }
    };
}

assert_default_hashing_controls!();