macro_rules! deps {
    () => {
        UnicodeWords!();
        WordsIter!();
    };
}

macro_rules! new_unicode_words {
    () => {
        deps!();
        # [inline] pub fn new_unicode_words (s : & str) -> UnicodeWords < '_ > { let inner = if s . is_ascii () { WordsIter :: Ascii (new_unicode_words_ascii (s)) } else { WordsIter :: Unicode (new_unicode_words_general (s)) } ; UnicodeWords { inner } }
    };
}

new_unicode_words!();