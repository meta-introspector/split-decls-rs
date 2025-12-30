// Generated macro for Ignore (struct)
macro_rules! Depcrate_format_description_modifierIgnore {
() => {
// Module: crate::format_description::modifier
// Provides: {"Ignore"}
// Dependencies: {}
# [doc = " Ignore some number of bytes."] # [doc = ""] # [doc = " This has no effect when formatting."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Ignore { # [doc = " The number of bytes to ignore."] pub count : NonZero < u16 > , }
};
}
