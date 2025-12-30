// Generated macro for UnifiedDiffHunk (struct)
macro_rules! Depcrate_udiffUnifiedDiffHunk {
() => {
// Module: crate::udiff
// Provides: {"UnifiedDiffHunk"}
// Dependencies: {}
# [doc = " Unified diff hunk formatter."] # [doc = ""] # [doc = " The `Display` this renders out a single unified diff's hunk."] pub struct UnifiedDiffHunk < 'diff , 'old , 'new , 'bufs , T : DiffableStr + ? Sized > { diff : & 'diff TextDiff < 'old , 'new , 'bufs , T > , ops : Vec < DiffOp > , missing_newline_hint : bool , }
};
}
