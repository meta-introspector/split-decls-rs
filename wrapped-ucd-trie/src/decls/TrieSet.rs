macro_rules! deps {
    () => {
        TrieSetSlice!();
    };
}

macro_rules! TrieSet {
    () => {
        deps!();
        # [doc = " A type alias for `TrieSetSlice<'static>`."] pub type TrieSet = TrieSetSlice < 'static > ;
    };
}

TrieSet!();