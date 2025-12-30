// Generated macro for impl_116 (impl)
macro_rules! Depcrate_diagnostics_utilsimpl_116 {
() => {
// Module: crate::diagnostics::utils
// Provides: {"impl_116"}
// Dependencies: {}
impl SuggestionKind { pub (crate) fn to_suggestion_style (& self) -> TokenStream { match self { SuggestionKind :: Normal => { quote ! { rustc_errors :: SuggestionStyle :: ShowCode } } SuggestionKind :: Short => { quote ! { rustc_errors :: SuggestionStyle :: HideCodeInline } } SuggestionKind :: Hidden => { quote ! { rustc_errors :: SuggestionStyle :: HideCodeAlways } } SuggestionKind :: Verbose => { quote ! { rustc_errors :: SuggestionStyle :: ShowAlways } } SuggestionKind :: ToolOnly => { quote ! { rustc_errors :: SuggestionStyle :: CompletelyHidden } } } } fn from_suffix (s : & str) -> Option < Self > { match s { "" => Some (SuggestionKind :: Normal) , "_short" => Some (SuggestionKind :: Short) , "_hidden" => Some (SuggestionKind :: Hidden) , "_verbose" => Some (SuggestionKind :: Verbose) , _ => None , } } }
};
}
