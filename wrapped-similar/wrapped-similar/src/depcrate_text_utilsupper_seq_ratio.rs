// Generated macro for upper_seq_ratio (function)
macro_rules! Depcrate_text_utilsupper_seq_ratio {
() => {
// Module: crate::text::utils
// Provides: {"upper_seq_ratio"}
// Dependencies: {}
pub fn upper_seq_ratio < T : PartialEq > (seq1 : & [T] , seq2 : & [T]) -> f32 { let n = seq1 . len () + seq2 . len () ; if n == 0 { 1.0 } else { 2.0 * seq1 . len () . min (seq2 . len ()) as f32 / n as f32 } }
};
}
