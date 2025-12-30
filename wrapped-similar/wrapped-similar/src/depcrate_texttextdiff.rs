// Generated macro for TextDiff (struct)
macro_rules! Depcrate_textTextDiff {
() => {
// Module: crate::text
// Provides: {"TextDiff"}
// Dependencies: {}
# [doc = " Captures diff op codes for textual diffs."] # [doc = ""] # [doc = " The exact diff behavior is depending on the underlying [`DiffableStr`]."] # [doc = " For instance diffs on bytes and strings are slightly different.  You can"] # [doc = " create a text diff from constructors such as [`TextDiff::from_lines`] or"] # [doc = " the [`TextDiffConfig`] created by [`TextDiff::configure`]."] # [doc = ""] # [doc = " Requires the `text` feature."] pub struct TextDiff < 'old , 'new , 'bufs , T : DiffableStr + ? Sized > { old : Cow < 'bufs , [& 'old T] > , new : Cow < 'bufs , [& 'new T] > , ops : Vec < DiffOp > , newline_terminated : bool , algorithm : Algorithm , }
};
}
