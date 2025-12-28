macro_rules! deps {
    () => {
        USentenceBoundIndices!();
    };
}

macro_rules! new_sentence_bound_indices {
    () => {
        deps!();
        # [inline] pub fn new_sentence_bound_indices (source : & str) -> USentenceBoundIndices < '_ > { USentenceBoundIndices { start_offset : source . as_ptr () as usize , iter : new_sentence_bounds (source) , } }
    };
}

new_sentence_bound_indices!();