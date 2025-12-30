// Generated macro for Word (struct)
macro_rules! Depcrate_coreWord {
() => {
// Module: crate::core
// Provides: {"Word"}
// Dependencies: {}
# [doc = " A piece of wrappable text, including any trailing whitespace."] # [doc = ""] # [doc = " A `Word` is an example of a [`Fragment`], so it has a width,"] # [doc = " trailing whitespace, and potentially a penalty item."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub struct Word < 'a > { # [doc = " Word content."] pub word : & 'a str , # [doc = " Whitespace to insert if the word does not fall at the end of a line."] pub whitespace : & 'a str , # [doc = " Penalty string to insert if the word falls at the end of a line."] pub penalty : & 'a str , # [doc = " Cached width in columns."] pub width : usize , }
};
}
