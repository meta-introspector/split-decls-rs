macro_rules! deps {
    () => {
        IndicesIter!();
        UnicodeWordIndices!();
    };
}

macro_rules! new_unicode_word_indices {
    () => {
        deps!();
        # [inline] pub fn new_unicode_word_indices (s : & str) -> UnicodeWordIndices < '_ > { let inner = if s . is_ascii () { IndicesIter :: Ascii (new_ascii_word_bound_indices (s) . filter (ascii_word_ok)) } else { IndicesIter :: Unicode (new_word_bound_indices (s) . filter (unicode_word_ok)) } ; UnicodeWordIndices { inner } }
    };
}

new_unicode_word_indices!();