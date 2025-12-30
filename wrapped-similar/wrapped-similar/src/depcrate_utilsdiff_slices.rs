// Generated macro for diff_slices (function)
macro_rules! Depcrate_utilsdiff_slices {
() => {
// Module: crate::utils
// Provides: {"diff_slices"}
// Dependencies: {}
# [doc = " Shortcut for diffing two slices."] # [doc = ""] # [doc = " This function produces the diff of two slices and returns a vector"] # [doc = " with the changes."] # [doc = ""] # [doc = " ```rust"] # [doc = " use similar::{Algorithm, ChangeTag};"] # [doc = " use similar::utils::diff_slices;"] # [doc = ""] # [doc = " let old = \"foo\\nbar\\nbaz\".lines().collect::<Vec<_>>();"] # [doc = " let new = \"foo\\nbar\\nBAZ\".lines().collect::<Vec<_>>();"] # [doc = " assert_eq!(diff_slices(Algorithm::Myers, &old, &new), vec!["] # [doc = "     (ChangeTag::Equal, &[\"foo\", \"bar\"][..]),"] # [doc = "     (ChangeTag::Delete, &[\"baz\"][..]),"] # [doc = "     (ChangeTag::Insert, &[\"BAZ\"][..]),"] # [doc = " ]);"] # [doc = " ```"] pub fn diff_slices < 'x , T : PartialEq + Hash + Ord > (alg : Algorithm , old : & 'x [T] , new : & 'x [T] ,) -> Vec < (ChangeTag , & 'x [T]) > { capture_diff_slices (alg , old , new) . iter () . flat_map (| op | op . iter_slices (old , new)) . collect () }
};
}
