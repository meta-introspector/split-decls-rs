// Generated macro for maybe_suggest_dashes (function)
macro_rules! Depcrate_unstable_bookmaybe_suggest_dashes {
() => {
// Module: crate::unstable_book
// Provides: {"maybe_suggest_dashes"}
// Dependencies: {}
# [doc = " Would switching underscores for dashes work?"] fn maybe_suggest_dashes (names : & BTreeSet < String > , feature_name : & str , bad : & mut bool) { let with_dashes = feature_name . replace ('_' , "-") ; if names . contains (& with_dashes) { tidy_error ! (bad , "the file `{}.md` contains underscores; use dashes instead: `{}.md`" , feature_name , with_dashes ,) ; } }
};
}
