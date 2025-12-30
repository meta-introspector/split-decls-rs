// Generated macro for sort_by_words (function)
macro_rules! Depcrate_edit_distancesort_by_words {
() => {
// Module: crate::edit_distance
// Provides: {"sort_by_words"}
// Dependencies: {}
fn sort_by_words (name : & str) -> Vec < & str > { let mut split_words : Vec < & str > = name . split ('_') . collect () ; split_words . sort_unstable () ; split_words }
};
}
