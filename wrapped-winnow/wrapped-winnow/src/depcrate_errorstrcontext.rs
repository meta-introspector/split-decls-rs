// Generated macro for StrContext (enum)
macro_rules! Depcrate_errorStrContext {
() => {
// Module: crate::error
// Provides: {"StrContext"}
// Dependencies: {}
# [doc = " Additional parse context for [`ContextError`] added via [`Parser::context`]"] # [derive (Clone , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum StrContext { # [doc = " Description of what is currently being parsed"] Label (& 'static str) , # [doc = " Grammar item that was expected"] Expected (StrContextValue) , }
};
}
