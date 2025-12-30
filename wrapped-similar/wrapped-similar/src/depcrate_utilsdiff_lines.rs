// Generated macro for diff_lines (function)
macro_rules! Depcrate_utilsdiff_lines {
() => {
// Module: crate::utils
// Provides: {"diff_lines"}
// Dependencies: {}
# [doc = " Shortcut for making a line diff."] # [doc = ""] # [doc = " This function produces the diff of two slices and returns a vector"] # [doc = " with the changes.  Unlike [`diff_chars`] or [`diff_slices`] it returns a"] # [doc = " change tag for each line."] # [doc = ""] # [doc = " ```rust"] # [doc = " use similar::{Algorithm, ChangeTag};"] # [doc = " use similar::utils::diff_lines;"] # [doc = ""] # [doc = " assert_eq!(diff_lines(Algorithm::Myers, \"foo\\nbar\\nbaz\\nblah\", \"foo\\nbar\\nbaz\\nblurgh\"), vec!["] # [doc = "     (ChangeTag::Equal, \"foo\\n\"),"] # [doc = "     (ChangeTag::Equal, \"bar\\n\"),"] # [doc = "     (ChangeTag::Equal, \"baz\\n\"),"] # [doc = "     (ChangeTag::Delete, \"blah\"),"] # [doc = "     (ChangeTag::Insert, \"blurgh\"),"] # [doc = " ]);"] # [doc = " ```"] pub fn diff_lines < 'x , T : DiffableStrRef + ? Sized > (alg : Algorithm , old : & 'x T , new : & 'x T ,) -> Vec < (ChangeTag , & 'x T :: Output) > { TextDiff :: configure () . algorithm (alg) . diff_lines (old , new) . iter_all_changes () . map (| change | (change . tag () , change . value ())) . collect () }
};
}
