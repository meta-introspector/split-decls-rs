// Generated macro for ExtraCheckParseError (enum)
macro_rules! Depcrate_extra_checksExtraCheckParseError {
() => {
// Module: crate::extra_checks
// Provides: {"ExtraCheckParseError"}
// Dependencies: {}
# [derive (Debug)] enum ExtraCheckParseError { # [allow (dead_code , reason = "shown through Debug")] UnknownKind (String) , # [allow (dead_code)] UnknownLang (String) , UnsupportedKindForLang , # [doc = " Too many `:`"] TooManyParts , # [doc = " Tried to parse the empty string"] Empty , # [doc = " `auto` specified without lang part."] AutoRequiresLang , }
};
}
