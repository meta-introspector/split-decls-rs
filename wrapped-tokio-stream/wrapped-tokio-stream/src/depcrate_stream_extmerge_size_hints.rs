// Generated macro for merge_size_hints (function)
macro_rules! Depcrate_stream_extmerge_size_hints {
() => {
// Module: crate::stream_ext
// Provides: {"merge_size_hints"}
// Dependencies: {}
# [doc = " Merge the size hints from two streams."] fn merge_size_hints ((left_low , left_high) : (usize , Option < usize >) , (right_low , right_high) : (usize , Option < usize >) ,) -> (usize , Option < usize >) { let low = left_low . saturating_add (right_low) ; let high = match (left_high , right_high) { (Some (h1) , Some (h2)) => h1 . checked_add (h2) , _ => None , } ; (low , high) }
};
}
