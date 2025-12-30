// Generated macro for CommentCodeSlices (struct)
macro_rules! Depcrate_commentCommentCodeSlices {
() => {
// Module: crate::comment
// Provides: {"CommentCodeSlices"}
// Dependencies: {}
# [doc = " Iterator over an alternating sequence of functional and commented parts of"] # [doc = " a string. The first item is always a, possibly zero length, subslice of"] # [doc = " functional text. Line style comments contain their ending newlines."] pub (crate) struct CommentCodeSlices < 'a > { slice : & 'a str , last_slice_kind : CodeCharKind , last_slice_end : usize , }
};
}
