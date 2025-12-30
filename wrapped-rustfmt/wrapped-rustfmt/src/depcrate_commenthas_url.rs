// Generated macro for has_url (function)
macro_rules! Depcrate_commenthas_url {
() => {
// Module: crate::comment
// Provides: {"has_url"}
// Dependencies: {}
# [doc = " Returns `true` if the given string MAY include URLs or alike."] fn has_url (s : & str) -> bool { let reference_link_url = static_regex ! (r"^\[.+\]\s?:") ; s . contains ("https://") || s . contains ("http://") || s . contains ("ftp://") || s . contains ("file://") || reference_link_url . is_match (s) }
};
}
