// Generated macro for WordBreakType (trait)
macro_rules! Depcrate_wordWordBreakType {
() => {
// Module: crate::word
// Provides: {"WordBreakType"}
// Dependencies: {}
# [doc = " A trait allowing for [`WordBreakIterator`] to be generalized to multiple string iteration methods."] # [doc = ""] # [doc = " This is implemented by ICU4X for several common string types."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this"] # [doc = " trait, please consider using a type from the implementors listed below."] # [doc = " </div>"] pub trait WordBreakType : crate :: private :: Sealed + Sized + RuleBreakType { # [doc (hidden)] fn word_handle_complex_language (iterator : & mut RuleBreakIterator < '_ , '_ , Self > , left_codepoint : Self :: CharType ,) -> Option < usize > ; }
};
}
