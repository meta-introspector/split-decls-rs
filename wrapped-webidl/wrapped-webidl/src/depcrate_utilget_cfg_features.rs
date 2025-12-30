// Generated macro for get_cfg_features (function)
macro_rules! Depcrate_utilget_cfg_features {
() => {
// Module: crate::util
// Provides: {"get_cfg_features"}
// Dependencies: {}
pub fn get_cfg_features (options : & Options , features : & BTreeSet < String >) -> Option < syn :: Attribute > { let len = features . len () ; if ! options . features || len == 0 { None } else { let features = features . iter () . map (| feature | quote ! (feature = # feature ,)) . collect :: < TokenStream > () ; if len == 1 { Some (syn :: parse_quote ! (# [cfg (# features)])) } else { Some (syn :: parse_quote ! (# [cfg (all (# features))])) } } }
};
}
