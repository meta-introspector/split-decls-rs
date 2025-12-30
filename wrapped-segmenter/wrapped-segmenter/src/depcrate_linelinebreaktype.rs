// Generated macro for LineBreakType (trait)
macro_rules! Depcrate_lineLineBreakType {
() => {
// Module: crate::line
// Provides: {"LineBreakType"}
// Dependencies: {}
# [doc = " A trait allowing for LineBreakIterator to be generalized to multiple string iteration methods."] # [doc = ""] # [doc = " This is implemented by ICU4X for several common string types."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this"] # [doc = " trait, please consider using a type from the implementors listed below."] # [doc = " </div>"] pub trait LineBreakType : crate :: private :: Sealed + Sized + RuleBreakType { # [doc (hidden)] fn use_complex_breaking (iterator : & LineBreakIterator < '_ , '_ , Self > , c : Self :: CharType) -> bool ; # [doc (hidden)] fn get_linebreak_property_with_rule (iterator : & LineBreakIterator < '_ , '_ , Self > , c : Self :: CharType ,) -> u8 ; # [doc (hidden)] fn line_handle_complex_language (iterator : & mut LineBreakIterator < '_ , '_ , Self > , left_codepoint : Self :: CharType ,) -> Option < usize > ; }
};
}
