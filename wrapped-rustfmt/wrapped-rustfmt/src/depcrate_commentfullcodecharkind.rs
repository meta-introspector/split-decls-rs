// Generated macro for FullCodeCharKind (enum)
macro_rules! Depcrate_commentFullCodeCharKind {
() => {
// Module: crate::comment
// Provides: {"FullCodeCharKind"}
// Dependencies: {}
# [doc = " Distinguish between functional part of code and comments,"] # [doc = " describing opening and closing of comments for ease when chunking"] # [doc = " code from tagged characters"] # [derive (PartialEq , Eq , Debug , Clone , Copy)] pub (crate) enum FullCodeCharKind { Normal , # [doc = " The first character of a comment, there is only one for a comment (always '/')"] StartComment , # [doc = " Any character inside a comment including the second character of comment"] # [doc = " marks (\"//\", \"/*\")"] InComment , # [doc = " Last character of a comment, '\\n' for a line comment, '/' for a block comment."] EndComment , # [doc = " Start of a multiline string inside a comment"] StartStringCommented , # [doc = " End of a multiline string inside a comment"] EndStringCommented , # [doc = " Inside a commented string"] InStringCommented , # [doc = " Start of a multiline string"] StartString , # [doc = " End of a multiline string"] EndString , # [doc = " Inside a string."] InString , }
};
}
