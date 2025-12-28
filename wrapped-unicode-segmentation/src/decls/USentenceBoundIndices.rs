macro_rules! deps {
    () => {
        USentenceBounds!();
        UnicodeSegmentation!();
    };
}

macro_rules! USentenceBoundIndices {
    () => {
        deps!();
        # [doc = " External iterator for sentence boundaries and byte offsets."] # [doc = ""] # [doc = " This struct is created by the [`split_sentence_bound_indices`] method on the"] # [doc = " [`UnicodeSegmentation`] trait. See its documentation for more."] # [doc = ""] # [doc = " [`split_sentence_bound_indices`]: trait.UnicodeSegmentation.html#tymethod.split_sentence_bound_indices"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug , Clone)] pub struct USentenceBoundIndices < 'a > { start_offset : usize , iter : USentenceBounds < 'a > , }
    };
}

USentenceBoundIndices!();