// Generated macro for StrContextValue (enum)
macro_rules! Depcrate_errorStrContextValue {
() => {
// Module: crate::error
// Provides: {"StrContextValue"}
// Dependencies: {}
# [doc = " See [`StrContext`]"] # [derive (Clone , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum StrContextValue { # [doc = " A [`char`] token"] CharLiteral (char) , # [doc = " A [`&str`] token"] StringLiteral (& 'static str) , # [doc = " A description of what was being parsed"] Description (& 'static str) , }
};
}
