// Generated macro for impl_85 (impl)
macro_rules! Depcrate_word_separatorsimpl_85 {
() => {
// Module: crate::word_separators
// Provides: {"impl_85"}
// Dependencies: {}
impl WordSeparator { # [doc = " Create a new word separator."] # [doc = ""] # [doc = " The best available algorithm is used by default, i.e.,"] # [doc = " [`WordSeparator::UnicodeBreakProperties`] if available,"] # [doc = " otherwise [`WordSeparator::AsciiSpace`]."] pub const fn new () -> Self { # [cfg (feature = "unicode-linebreak")] { WordSeparator :: UnicodeBreakProperties } # [cfg (not (feature = "unicode-linebreak"))] { WordSeparator :: AsciiSpace } } # [doc = " Find all words in `line`."] pub fn find_words < 'a > (& self , line : & 'a str) -> Box < dyn Iterator < Item = Word < 'a > > + 'a > { match self { WordSeparator :: AsciiSpace => find_words_ascii_space (line) , # [cfg (feature = "unicode-linebreak")] WordSeparator :: UnicodeBreakProperties => find_words_unicode_break_properties (line) , WordSeparator :: Custom (func) => func (line) , } } }
};
}
