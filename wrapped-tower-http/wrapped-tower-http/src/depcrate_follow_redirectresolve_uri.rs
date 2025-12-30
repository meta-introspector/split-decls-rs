// Generated macro for resolve_uri (function)
macro_rules! Depcrate_follow_redirectresolve_uri {
() => {
// Module: crate::follow_redirect
// Provides: {"resolve_uri"}
// Dependencies: {}
# [doc = " Try to resolve a URI reference `relative` against a base URI `base`."] fn resolve_uri (relative : & str , base : & Uri) -> Option < Uri > { let relative = UriReferenceStr :: new (relative) . ok () ? ; let base = UriAbsoluteString :: try_from (base . to_string ()) . ok () ? ; let uri = relative . resolve_against (& base) . to_string () ; Uri :: try_from (uri) . ok () }
};
}
