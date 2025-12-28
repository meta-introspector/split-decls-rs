macro_rules! BUCKETS {
    () => {
        # [doc = " The total number of buckets stored in each thread local."] # [doc = " All buckets combined can hold up to `usize::MAX - 1` entries."] const BUCKETS : usize = (usize :: BITS - 1) as usize ;
    };
}

BUCKETS!();