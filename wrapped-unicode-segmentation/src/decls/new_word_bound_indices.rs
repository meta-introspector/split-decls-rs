macro_rules! deps {
    () => {
        UWordBoundIndices!();
    };
}

macro_rules! new_word_bound_indices {
    () => {
        deps!();
        # [inline] pub fn new_word_bound_indices (s : & str) -> UWordBoundIndices < '_ > { UWordBoundIndices { start_offset : s . as_ptr () as usize , iter : new_word_bounds (s) , } }
    };
}

new_word_bound_indices!()