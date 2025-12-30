// Generated macro for impl_115 (impl)
macro_rules! Depcrate_diagnostics_utilsimpl_115 {
() => {
// Module: crate::diagnostics::utils
// Provides: {"impl_115"}
// Dependencies: {}
impl fmt :: Display for SuggestionKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { SuggestionKind :: Normal => write ! (f , "normal") , SuggestionKind :: Short => write ! (f , "short") , SuggestionKind :: Hidden => write ! (f , "hidden") , SuggestionKind :: Verbose => write ! (f , "verbose") , SuggestionKind :: ToolOnly => write ! (f , "tool-only") , } } }
};
}
