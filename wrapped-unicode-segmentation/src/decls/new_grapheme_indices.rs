macro_rules! deps {
    () => {
        GraphemeIndices!();
    };
}

macro_rules! new_grapheme_indices {
    () => {
        deps!();
        # [inline] pub fn new_grapheme_indices (s : & str , is_extended : bool) -> GraphemeIndices < '_ > { GraphemeIndices { start_offset : s . as_ptr () as usize , iter : new_graphemes (s , is_extended) , } }
    };
}

new_grapheme_indices!()