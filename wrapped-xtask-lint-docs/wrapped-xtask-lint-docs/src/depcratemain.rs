// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> anyhow :: Result < () > { let args = cli () . get_matches () ; let check = args . flag ("check") ; let mut allow = Vec :: new () ; let mut warn = Vec :: new () ; let mut deny = Vec :: new () ; let mut forbid = Vec :: new () ; let mut lint_docs = String :: new () ; for lint in cargo :: util :: lints :: LINTS . iter () . sorted_by_key (| lint | lint . name) { if lint . docs . is_some () { let sectipn = match lint . default_level { LintLevel :: Allow => & mut allow , LintLevel :: Warn => & mut warn , LintLevel :: Deny => & mut deny , LintLevel :: Forbid => & mut forbid , } ; sectipn . push (lint . name) ; add_lint (lint , & mut lint_docs) ? ; } } let mut buf = String :: new () ; writeln ! (buf , "# Lints\n") ? ; writeln ! (buf , "Note: [Cargo's linting system is unstable](unstable.md#lintscargo) and can only be used on nightly toolchains") ? ; writeln ! (buf) ? ; if ! allow . is_empty () { add_level_section (LintLevel :: Allow , & allow , & mut buf) ? ; } if ! warn . is_empty () { add_level_section (LintLevel :: Warn , & warn , & mut buf) ? ; } if ! deny . is_empty () { add_level_section (LintLevel :: Deny , & deny , & mut buf) ? ; } if ! forbid . is_empty () { add_level_section (LintLevel :: Forbid , & forbid , & mut buf) ? ; } buf . push_str (& lint_docs) ; if check { let old = std :: fs :: read_to_string (lint_docs_path ()) ? ; if old != buf { anyhow :: bail ! ("The lints documentation is out-of-date. Run `cargo lint-docs` to update it.") ; } } else { std :: fs :: write (lint_docs_path () , buf) ? ; } Ok (()) }
};
}
