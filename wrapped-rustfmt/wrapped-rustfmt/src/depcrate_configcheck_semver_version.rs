// Generated macro for check_semver_version (function)
macro_rules! Depcrate_configcheck_semver_version {
() => {
// Module: crate::config
// Provides: {"check_semver_version"}
// Dependencies: {}
fn check_semver_version (range_requirement : & str , actual : & str) -> bool { let mut version_req = match semver :: VersionReq :: parse (range_requirement) { Ok (r) => r , Err (e) => { eprintln ! ("Error: failed to parse required version {range_requirement:?}: {e}") ; return false ; } } ; let actual_version = match semver :: Version :: parse (actual) { Ok (v) => v , Err (e) => { eprintln ! ("Error: failed to parse current version {actual:?}: {e}") ; return false ; } } ; range_requirement . split (',') . enumerate () . for_each (| (i , label) | { let Some (comparator) = version_req . comparators . get_mut (i) else { return ; } ; if ! label . starts_with ('^') && comparator . op == semver :: Op :: Caret { comparator . op = semver :: Op :: Exact ; } }) ; version_req . matches (& actual_version) }
};
}
