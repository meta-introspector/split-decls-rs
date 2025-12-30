// Generated macro for diff_graphemes (function)
macro_rules! Depcrate_utilsdiff_graphemes {
() => {
// Module: crate::utils
// Provides: {"diff_graphemes"}
// Dependencies: {}
# [doc = " Shortcut for making a grapheme level diff."] # [doc = ""] # [doc = " This function produces the diff of two strings and returns a vector"] # [doc = " with the changes.  It returns connected slices into the original string"] # [doc = " rather than grapheme level slices."] # [doc = ""] # [doc = " ```rust"] # [doc = " use similar::{Algorithm, ChangeTag};"] # [doc = " use similar::utils::diff_graphemes;"] # [doc = ""] # [doc = " let old = \"The flag of Austria is 🇦🇹\";"] # [doc = " let new = \"The flag of Albania is 🇦🇱\";"] # [doc = " assert_eq!(diff_graphemes(Algorithm::Myers, old, new), vec!["] # [doc = "     (ChangeTag::Equal, \"The flag of A\"),"] # [doc = "     (ChangeTag::Delete, \"ustr\"),"] # [doc = "     (ChangeTag::Insert, \"lban\"),"] # [doc = "     (ChangeTag::Equal, \"ia is \"),"] # [doc = "     (ChangeTag::Delete, \"🇦🇹\"),"] # [doc = "     (ChangeTag::Insert, \"🇦🇱\"),"] # [doc = " ]);"] # [doc = " ```"] # [doc = ""] # [doc = " This requires the `unicode` feature."] # [cfg (feature = "unicode")] pub fn diff_graphemes < 'x , T : DiffableStrRef + ? Sized > (alg : Algorithm , old : & 'x T , new : & 'x T ,) -> Vec < (ChangeTag , & 'x T :: Output) > { let old = old . as_diffable_str () ; let new = new . as_diffable_str () ; let diff = TextDiff :: configure () . algorithm (alg) . diff_graphemes (old , new) ; let remapper = TextDiffRemapper :: from_text_diff (& diff , old , new) ; diff . ops () . iter () . flat_map (move | x | remapper . iter_slices (x)) . collect () }
};
}
