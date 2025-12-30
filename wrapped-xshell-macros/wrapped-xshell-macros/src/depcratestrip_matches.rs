// Generated macro for strip_matches (function)
macro_rules! Depcratestrip_matches {
() => {
// Module: crate
// Provides: {"strip_matches"}
// Dependencies: {}
# [doc = " Like trim_matches except only trims a maximum of 1 match"] fn strip_matches < 'a > (s : & 'a str , pattern : & str) -> & 'a str { s . strip_prefix (pattern) . unwrap_or (s) . strip_suffix (pattern) . unwrap_or (s) }
};
}
