// Generated macro for diff_words (function)
macro_rules! Depcrate_utilsdiff_words {
() => {
// Module: crate::utils
// Provides: {"diff_words"}
// Dependencies: {}
# [doc = " Shortcut for making a word level diff."] # [doc = ""] # [doc = " This function produces the diff of two strings and returns a vector"] # [doc = " with the changes.  It returns connected slices into the original string"] # [doc = " rather than word level slices."] # [doc = ""] # [doc = " ```rust"] # [doc = " use similar::{Algorithm, ChangeTag};"] # [doc = " use similar::utils::diff_words;"] # [doc = ""] # [doc = " assert_eq!(diff_words(Algorithm::Myers, \"foo bar baz\", \"foo bor baz\"), vec!["] # [doc = "     (ChangeTag::Equal, \"foo \"),"] # [doc = "     (ChangeTag::Delete, \"bar\"),"] # [doc = "     (ChangeTag::Insert, \"bor\"),"] # [doc = "     (ChangeTag::Equal, \" baz\"),"] # [doc = " ]);"] # [doc = " ```"] pub fn diff_words < 'x , T : DiffableStrRef + ? Sized > (alg : Algorithm , old : & 'x T , new : & 'x T ,) -> Vec < (ChangeTag , & 'x T :: Output) > { let old = old . as_diffable_str () ; let new = new . as_diffable_str () ; let diff = TextDiff :: configure () . algorithm (alg) . diff_words (old , new) ; let remapper = TextDiffRemapper :: from_text_diff (& diff , old , new) ; diff . ops () . iter () . flat_map (move | x | remapper . iter_slices (x)) . collect () }
};
}
