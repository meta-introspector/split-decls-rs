// Generated macro for tests (module)
macro_rules! Depcrate_serdetests {
() => {
// Module: crate::serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_serde_process_name () { if ! crate :: IS_SUPPORTED_SYSTEM { return ; } let mut s = crate :: System :: new () ; s . refresh_processes_specifics (crate :: ProcessesToUpdate :: All , false , crate :: ProcessRefreshKind :: nothing () ,) ; if s . processes () . is_empty () { panic ! ("no processes?") ; } for p in s . processes () . values () { let values = match serde_json :: to_value (p) { Ok (serde_json :: Value :: Object (values)) => values , other => panic ! ("expected object, found `{other:?}`") , } ; match values . get ("name") { Some (serde_json :: Value :: String (_)) => { } value => panic ! ("expected a string, found `{value:?}`") , } } } }
};
}
