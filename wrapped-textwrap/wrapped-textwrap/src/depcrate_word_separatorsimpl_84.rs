// Generated macro for impl_84 (impl)
macro_rules! Depcrate_word_separatorsimpl_84 {
() => {
// Module: crate::word_separators
// Provides: {"impl_84"}
// Dependencies: {}
impl PartialEq for WordSeparator { # [doc = " Compare two word separators."] # [doc = ""] # [doc = " ```"] # [doc = " use textwrap::WordSeparator;"] # [doc = ""] # [doc = " assert_eq!(WordSeparator::AsciiSpace, WordSeparator::AsciiSpace);"] # [doc = " #[cfg(feature = \"unicode-linebreak\")] {"] # [doc = "     assert_eq!(WordSeparator::UnicodeBreakProperties,"] # [doc = "                WordSeparator::UnicodeBreakProperties);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Note that `WordSeparator::Custom` values never compare equal:"] # [doc = ""] # [doc = " ```"] # [doc = " use textwrap::WordSeparator;"] # [doc = " use textwrap::core::Word;"] # [doc = " fn word_separator(line: &str) -> Box<dyn Iterator<Item = Word<'_>> + '_> {"] # [doc = "     Box::new(line.split_inclusive(' ').map(Word::from))"] # [doc = " }"] # [doc = " assert_ne!(WordSeparator::Custom(word_separator),"] # [doc = "            WordSeparator::Custom(word_separator));"] # [doc = " ```"] fn eq (& self , other : & Self) -> bool { match (self , other) { (WordSeparator :: AsciiSpace , WordSeparator :: AsciiSpace) => true , # [cfg (feature = "unicode-linebreak")] (WordSeparator :: UnicodeBreakProperties , WordSeparator :: UnicodeBreakProperties) => true , (_ , _) => false , } } }
};
}
