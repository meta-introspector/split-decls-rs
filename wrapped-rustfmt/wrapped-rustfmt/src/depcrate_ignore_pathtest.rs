// Generated macro for test (module)
macro_rules! Depcrate_ignore_pathtest {
() => {
// Module: crate::ignore_path
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use rustfmt_config_proc_macro :: nightly_only_test ; # [nightly_only_test] # [test] fn test_ignore_path_set () { use crate :: config :: { Config , FileName } ; use crate :: ignore_path :: IgnorePathSet ; use std :: path :: { Path , PathBuf } ; let config = Config :: from_toml (r#"ignore = ["foo.rs", "bar_dir/*"]"# , Path :: new ("./rustfmt.toml") ,) . unwrap () ; let ignore_path_set = IgnorePathSet :: from_ignore_list (& config . ignore ()) . unwrap () ; assert ! (ignore_path_set . is_match (& FileName :: Real (PathBuf :: from ("src/foo.rs")))) ; assert ! (ignore_path_set . is_match (& FileName :: Real (PathBuf :: from ("bar_dir/baz.rs")))) ; assert ! (! ignore_path_set . is_match (& FileName :: Real (PathBuf :: from ("src/bar.rs")))) ; } # [nightly_only_test] # [test] fn test_negated_ignore_path_set () { use crate :: config :: { Config , FileName } ; use crate :: ignore_path :: IgnorePathSet ; use std :: path :: { Path , PathBuf } ; let config = Config :: from_toml (r#"ignore = ["foo.rs", "bar_dir/*", "!bar_dir/*/what.rs"]"# , Path :: new ("./rustfmt.toml") ,) . unwrap () ; let ignore_path_set = IgnorePathSet :: from_ignore_list (& config . ignore ()) . unwrap () ; assert ! (ignore_path_set . is_match (& FileName :: Real (PathBuf :: from ("bar_dir/what.rs")))) ; assert ! (ignore_path_set . is_match (& FileName :: Real (PathBuf :: from ("bar_dir/baz/a.rs")))) ; assert ! (! ignore_path_set . is_match (& FileName :: Real (PathBuf :: from ("bar_dir/baz/what.rs")))) ; } }
};
}
