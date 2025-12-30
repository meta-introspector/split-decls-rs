// Generated macro for check (function)
macro_rules! Depcrate_extdepscheck {
() => {
// Module: crate::extdeps
// Provides: {"check"}
// Dependencies: {}
# [doc = " Checks for external package sources. `root` is the path to the directory that contains the"] # [doc = " workspace `Cargo.toml`."] pub fn check (root : & Path , bad : & mut bool) { for & (workspace , _ , _ , submodules) in crate :: deps :: WORKSPACES { if crate :: deps :: has_missing_submodule (root , submodules) { continue ; } let path = root . join (workspace) . join ("Cargo.lock") ; if ! path . exists () { tidy_error ! (bad , "the `{workspace}` workspace doesn't have a Cargo.lock") ; continue ; } let cargo_lock = t ! (fs :: read_to_string (& path)) ; for line in cargo_lock . lines () { if ! line . starts_with ("source = ") { continue ; } let source = line . split_once ('=') . unwrap () . 1 . trim () ; if ! ALLOWED_SOURCES . contains (& source) { tidy_error ! (bad , "invalid source: {}" , source) ; } } } }
};
}
