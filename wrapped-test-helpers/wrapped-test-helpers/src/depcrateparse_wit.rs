// Generated macro for parse_wit (function)
macro_rules! Depcrateparse_wit {
() => {
// Module: crate
// Provides: {"parse_wit"}
// Dependencies: {}
fn parse_wit (path : & Path) -> (Resolve , WorldId) { let mut resolve = Resolve :: default () ; let (pkg , _files) = resolve . push_path (path) . unwrap () ; let world = resolve . select_world (& [pkg] , None) . unwrap_or_else (| _ | { resolve . select_world (& [pkg] , Some ("imports")) . unwrap () }) ; (resolve , world) }
};
}
