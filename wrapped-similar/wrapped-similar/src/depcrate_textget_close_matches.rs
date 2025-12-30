// Generated macro for get_close_matches (function)
macro_rules! Depcrate_textget_close_matches {
() => {
// Module: crate::text
// Provides: {"get_close_matches"}
// Dependencies: {}
# [doc = " Use the text differ to find `n` close matches."] # [doc = ""] # [doc = " `cutoff` defines the threshold which needs to be reached for a word"] # [doc = " to be considered similar.  See [`TextDiff::ratio`] for more information."] # [doc = ""] # [doc = " ```"] # [doc = " # use similar::get_close_matches;"] # [doc = " let matches = get_close_matches("] # [doc = "     \"appel\","] # [doc = "     &[\"ape\", \"apple\", \"peach\", \"puppy\"][..],"] # [doc = "     3,"] # [doc = "     0.6"] # [doc = " );"] # [doc = " assert_eq!(matches, vec![\"apple\", \"ape\"]);"] # [doc = " ```"] # [doc = ""] # [doc = " Requires the `text` feature."] pub fn get_close_matches < 'a , T : DiffableStr + ? Sized > (word : & T , possibilities : & [& 'a T] , n : usize , cutoff : f32 ,) -> Vec < & 'a T > { let mut matches = BinaryHeap :: new () ; let seq1 = word . tokenize_chars () ; let quick_ratio = QuickSeqRatio :: new (& seq1) ; for & possibility in possibilities { let seq2 = possibility . tokenize_chars () ; if upper_seq_ratio (& seq1 , & seq2) < cutoff || quick_ratio . calc (& seq2) < cutoff { continue ; } let diff = TextDiff :: from_slices (& seq1 , & seq2) ; let ratio = diff . ratio () ; if ratio >= cutoff { matches . push (((ratio * u32 :: MAX as f32) as u32 , Reverse (possibility))) ; } } let mut rv = vec ! [] ; for _ in 0 .. n { if let Some ((_ , elt)) = matches . pop () { rv . push (elt . 0) ; } else { break ; } } rv }
};
}
