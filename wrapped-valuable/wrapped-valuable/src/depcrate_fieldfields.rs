// Generated macro for Fields (enum)
macro_rules! Depcrate_fieldFields {
() => {
// Module: crate::field
// Provides: {"Fields"}
// Dependencies: {}
# [doc = " Data stored within a `Structable` or  an `Enumerable`."] # [derive (Debug)] pub enum Fields < 'a > { # [doc = " Named fields"] Named (& 'a [NamedField < 'a >]) , # [doc = " Unnamed (positional) fields or unit"] # [doc = ""] # [doc = " The `usize` value represents the number of fields."] Unnamed (usize) , }
};
}
