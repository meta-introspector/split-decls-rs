// Generated macro for Suggestion (struct)
macro_rules! DepcrateSuggestion {
() => {
// Module: crate
// Provides: {"Suggestion"}
// Dependencies: {}
# [doc = " An error/warning and possible solutions for fixing it"] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Suggestion { pub message : String , pub snippets : Vec < Snippet > , pub solutions : Vec < Solution > , }
};
}
