// Generated macro for UngroupedCommentCodeSlices (struct)
macro_rules! Depcrate_commentUngroupedCommentCodeSlices {
() => {
// Module: crate::comment
// Provides: {"UngroupedCommentCodeSlices"}
// Dependencies: {}
# [doc = " Iterator over functional and commented parts of a string. Any part of a string is either"] # [doc = " functional code, either *one* block comment, either *one* line comment. Whitespace between"] # [doc = " comments is functional code. Line comments contain their ending newlines."] struct UngroupedCommentCodeSlices < 'a > { slice : & 'a str , iter : iter :: Peekable < CharClasses < std :: str :: CharIndices < 'a > > > , }
};
}
