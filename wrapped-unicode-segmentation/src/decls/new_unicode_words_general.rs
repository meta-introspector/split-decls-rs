macro_rules! deps {
    () => {
        UnicodeWordsIter!();
    };
}

macro_rules! new_unicode_words_general {
    () => {
        deps!();
        # [inline] fn new_unicode_words_general < 'a > (s : & 'a str) -> UnicodeWordsIter < 'a > { new_word_bounds (s) . filter (has_alphanumeric) }
    };
}

new_unicode_words_general!();