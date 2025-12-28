macro_rules! deps {
    () => {
        AsciiWordsIter!();
    };
}

macro_rules! new_unicode_words_ascii {
    () => {
        deps!();
        # [inline] fn new_unicode_words_ascii < 'a > (s : & 'a str) -> AsciiWordsIter < 'a > { new_ascii_word_bound_indices (s) . map (strip_pos as fn (_) -> _) . filter (has_ascii_alphanumeric) }
    };
}

new_unicode_words_ascii!();