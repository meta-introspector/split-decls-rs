// Generated macro for check (function)
macro_rules! Depcrate_editioncheck {
() => {
// Module: crate::edition
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , bad : & mut bool) { walk (path , | path , _is_dir | filter_dirs (path) , & mut | entry , contents | { let file = entry . path () ; let filename = file . file_name () . unwrap () ; if filename != "Cargo.toml" { return ; } let is_current_edition = contents . lines () . any (| line | line . trim () == "edition = \"2021\"" || line . trim () == "edition = \"2024\"") ; let is_workspace = contents . lines () . any (| line | line . trim () == "[workspace]") ; let is_package = contents . lines () . any (| line | line . trim () == "[package]") ; assert ! (is_workspace || is_package) ; if is_package && ! is_current_edition { tidy_error ! (bad , "{} doesn't have `edition = \"2021\"` or `edition = \"2024\"` on a separate line" , file . display ()) ; } }) ; }
};
}
