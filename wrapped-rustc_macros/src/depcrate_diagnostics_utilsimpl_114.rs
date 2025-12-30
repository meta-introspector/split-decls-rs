// Generated macro for impl_114 (impl)
macro_rules! Depcrate_diagnostics_utilsimpl_114 {
() => {
// Module: crate::diagnostics::utils
// Provides: {"impl_114"}
// Dependencies: {}
impl FromStr for SuggestionKind { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "normal" => Ok (SuggestionKind :: Normal) , "short" => Ok (SuggestionKind :: Short) , "hidden" => Ok (SuggestionKind :: Hidden) , "verbose" => Ok (SuggestionKind :: Verbose) , "tool-only" => Ok (SuggestionKind :: ToolOnly) , _ => Err (()) , } } }
};
}
