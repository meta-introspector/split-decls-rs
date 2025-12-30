// Generated macro for QuickSeqRatio (struct)
macro_rules! Depcrate_text_utilsQuickSeqRatio {
() => {
// Module: crate::text::utils
// Provides: {"QuickSeqRatio"}
// Dependencies: {}
# [doc = " Internal utility to calculate an upper bound for a ratio for"] # [doc = " [`get_close_matches`].  This is based on Python's difflib approach"] # [doc = " of considering the two sets to be multisets."] # [doc = ""] # [doc = " It counts the number of matches without regard to order, which is an"] # [doc = " obvious upper bound."] pub struct QuickSeqRatio < 'a , T : DiffableStrRef + ? Sized > (HashMap < & 'a T , i32 >) ;
};
}
