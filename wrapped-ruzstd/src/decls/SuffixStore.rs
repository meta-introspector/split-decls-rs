macro_rules! SuffixStore {
    () => {
        # [doc = " This stores the index of a suffix of a string by hashing the first few bytes of that suffix"] # [doc = " This means that collisions just overwrite and that you need to check validity after a get"] struct SuffixStore { slots : Vec < Option < NonZeroUsize > > , len_log : u32 , }
    };
}

SuffixStore!()