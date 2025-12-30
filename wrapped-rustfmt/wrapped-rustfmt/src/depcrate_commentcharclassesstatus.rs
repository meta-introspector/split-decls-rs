// Generated macro for CharClassesStatus (enum)
macro_rules! Depcrate_commentCharClassesStatus {
() => {
// Module: crate::comment
// Provides: {"CharClassesStatus"}
// Dependencies: {}
# [derive (PartialEq , Eq , Debug , Clone , Copy)] enum CharClassesStatus { Normal , # [doc = " Character is within a string"] LitString , LitStringEscape , # [doc = " Character is within a raw string"] LitRawString (u32) , RawStringPrefix (u32) , RawStringSuffix (u32) , LitChar , LitCharEscape , # [doc = " Character inside a block comment, with the integer indicating the nesting deepness of the"] # [doc = " comment"] BlockComment (u32) , # [doc = " Character inside a block-commented string, with the integer indicating the nesting deepness"] # [doc = " of the comment"] StringInBlockComment (u32) , # [doc = " Status when the '/' has been consumed, but not yet the '*', deepness is"] # [doc = " the new deepness (after the comment opening)."] BlockCommentOpening (u32) , # [doc = " Status when the '*' has been consumed, but not yet the '/', deepness is"] # [doc = " the new deepness (after the comment closing)."] BlockCommentClosing (u32) , # [doc = " Character is within a line comment"] LineComment , }
};
}
