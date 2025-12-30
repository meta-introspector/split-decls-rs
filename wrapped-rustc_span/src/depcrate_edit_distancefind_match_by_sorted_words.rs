// Generated macro for find_match_by_sorted_words (function)
macro_rules! Depcrate_edit_distancefind_match_by_sorted_words {
() => {
// Module: crate::edit_distance
// Provides: {"find_match_by_sorted_words"}
// Dependencies: {}
fn find_match_by_sorted_words (iter_names : & [Symbol] , lookup : & str) -> Option < Symbol > { let lookup_sorted_by_words = sort_by_words (lookup) ; iter_names . iter () . fold (None , | result , candidate | { if sort_by_words (candidate . as_str ()) == lookup_sorted_by_words { Some (* candidate) } else { result } }) }
};
}
