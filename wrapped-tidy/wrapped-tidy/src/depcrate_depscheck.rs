// Generated macro for check (function)
macro_rules! Depcrate_depscheck {
() => {
// Module: crate::deps
// Provides: {"check"}
// Dependencies: {}
# [doc = " Dependency checks."] # [doc = ""] # [doc = " `root` is path to the directory with the root `Cargo.toml` (for the workspace). `cargo` is path"] # [doc = " to the cargo executable."] pub fn check (root : & Path , cargo : & Path , bless : bool , bad : & mut bool) { let mut checked_runtime_licenses = false ; check_proc_macro_dep_list (root , cargo , bless , bad) ; for & (workspace , exceptions , permitted_deps , submodules) in WORKSPACES { if has_missing_submodule (root , submodules) { continue ; } if ! root . join (workspace) . join ("Cargo.lock") . exists () { tidy_error ! (bad , "the `{workspace}` workspace doesn't have a Cargo.lock") ; continue ; } let mut cmd = cargo_metadata :: MetadataCommand :: new () ; cmd . cargo_path (cargo) . manifest_path (root . join (workspace) . join ("Cargo.toml")) . features (cargo_metadata :: CargoOpt :: AllFeatures) . other_options (vec ! ["--locked" . to_owned ()]) ; let metadata = t ! (cmd . exec ()) ; check_license_exceptions (& metadata , workspace , exceptions , bad) ; if let Some ((crates , permitted_deps)) = permitted_deps { check_permitted_dependencies (& metadata , workspace , permitted_deps , crates , bad) ; } if workspace == "library" { check_runtime_license_exceptions (& metadata , bad) ; check_runtime_no_duplicate_dependencies (& metadata , bad) ; check_runtime_no_proc_macros (& metadata , bad) ; checked_runtime_licenses = true ; } } assert ! (checked_runtime_licenses) ; }
};
}
