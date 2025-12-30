// Generated macro for has_physical_root (function)
macro_rules! Depcrate_pathhas_physical_root {
() => {
// Module: crate::path
// Provides: {"has_physical_root"}
// Dependencies: {}
# [doc = " Says whether the first byte after the prefix is a separator."] fn has_physical_root (s : & [u8] , prefix : Option < Prefix < '_ > >) -> bool { let path = if let Some (p) = prefix { & s [p . len () ..] } else { s } ; ! path . is_empty () && is_sep_byte (path [0]) }
};
}
