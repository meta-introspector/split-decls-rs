// Generated macro for collect_unstable_feature_names (function)
macro_rules! Depcrate_unstable_bookcollect_unstable_feature_names {
() => {
// Module: crate::unstable_book
// Provides: {"collect_unstable_feature_names"}
// Dependencies: {}
# [doc = " Retrieves names of all unstable features."] pub fn collect_unstable_feature_names (features : & Features) -> BTreeSet < String > { features . iter () . filter (| & (_ , f) | f . level == Status :: Unstable) . map (| (name , _) | name . replace ('_' , "-")) . collect () }
};
}
