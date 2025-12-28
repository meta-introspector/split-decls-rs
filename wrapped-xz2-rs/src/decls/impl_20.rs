macro_rules! deps {
    () => {
        MatchFinder!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl MatchFinder { # [doc = " Test if this match finder is supported in this build of liblzma."] pub fn is_supported (& self) -> bool { let ret = unsafe { lzma_sys :: lzma_mf_is_supported (* self as lzma_sys :: lzma_match_finder) } ; ret != 0 } }
    };
}

impl_20!();