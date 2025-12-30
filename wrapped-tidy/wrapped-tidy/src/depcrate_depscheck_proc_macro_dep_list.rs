// Generated macro for check_proc_macro_dep_list (function)
macro_rules! Depcrate_depscheck_proc_macro_dep_list {
() => {
// Module: crate::deps
// Provides: {"check_proc_macro_dep_list"}
// Dependencies: {}
# [doc = " Ensure the list of proc-macro crate transitive dependencies is up to date"] fn check_proc_macro_dep_list (root : & Path , cargo : & Path , bless : bool , bad : & mut bool) { let mut cmd = cargo_metadata :: MetadataCommand :: new () ; cmd . cargo_path (cargo) . manifest_path (root . join ("Cargo.toml")) . features (cargo_metadata :: CargoOpt :: AllFeatures) . other_options (vec ! ["--locked" . to_owned ()]) ; let metadata = t ! (cmd . exec ()) ; let is_proc_macro_pkg = | pkg : & Package | pkg . targets . iter () . any (| target | target . is_proc_macro ()) ; let mut proc_macro_deps = HashSet :: new () ; for pkg in metadata . packages . iter () . filter (| pkg | is_proc_macro_pkg (pkg)) { deps_of (& metadata , & pkg . id , & mut proc_macro_deps) ; } proc_macro_deps . retain (| pkg | ! is_proc_macro_pkg (& metadata [pkg])) ; let proc_macro_deps : HashSet < _ > = proc_macro_deps . into_iter () . map (| dep | metadata [dep] . name . as_ref ()) . collect () ; let expected = proc_macro_deps :: CRATES . iter () . copied () . collect :: < HashSet < _ > > () ; let needs_blessing = proc_macro_deps . difference (& expected) . next () . is_some () || expected . difference (& proc_macro_deps) . next () . is_some () ; if needs_blessing && bless { let mut proc_macro_deps : Vec < _ > = proc_macro_deps . into_iter () . collect () ; proc_macro_deps . sort () ; let mut file = File :: create (root . join ("src/bootstrap/src/utils/proc_macro_deps.rs")) . expect ("`proc_macro_deps` should exist") ; writeln ! (& mut file , "/// Do not update manually - use `./x.py test tidy --bless`
/// Holds all direct and indirect dependencies of proc-macro crates in tree.
/// See <https://github.com/rust-lang/rust/issues/134863>
pub static CRATES: &[&str] = &[
    // tidy-alphabetical-start") . unwrap () ; for dep in proc_macro_deps { writeln ! (& mut file , "    {dep:?},") . unwrap () ; } writeln ! (& mut file , "    // tidy-alphabetical-end
];") . unwrap () ; } else { let old_bad = * bad ; for missing in proc_macro_deps . difference (& expected) { tidy_error ! (bad , "proc-macro crate dependency `{missing}` is not registered in `src/bootstrap/src/utils/proc_macro_deps.rs`" ,) ; } for extra in expected . difference (& proc_macro_deps) { tidy_error ! (bad , "`{extra}` is registered in `src/bootstrap/src/utils/proc_macro_deps.rs`, but is not a proc-macro crate dependency" ,) ; } if * bad != old_bad { eprintln ! ("Run `./x.py test tidy --bless` to regenerate the list") ; } } }
};
}
