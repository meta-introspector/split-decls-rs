// Generated macro for appendix_c_cmp (function)
macro_rules! Depcrate_locale_aliasesappendix_c_cmp {
() => {
// Module: crate::locale::aliases
// Provides: {"appendix_c_cmp"}
// Dependencies: {}
fn appendix_c_cmp (langid : & LanguageIdentifier) -> impl Ord { let mut union_size = langid . variants . len () as i8 ; if ! langid . language . is_unknown () { union_size += 1 ; } if langid . script . is_some () { union_size += 1 ; } if langid . region . is_some () { union_size += 1 ; } (- union_size , langid . language , langid . script , langid . region , langid . variants . clone () ,) }
};
}
