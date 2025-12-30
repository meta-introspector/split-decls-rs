// Generated macro for check (function)
macro_rules! Depcrate_gcc_submodulecheck {
() => {
// Module: crate::gcc_submodule
// Provides: {"check"}
// Dependencies: {}
pub fn check (root_path : & Path , compiler_path : & Path , bad : & mut bool) { let cg_gcc_version_path = compiler_path . join ("rustc_codegen_gcc/libgccjit.version") ; let cg_gcc_version = std :: fs :: read_to_string (& cg_gcc_version_path) . unwrap_or_else (| _ | { panic ! ("Cannot read GCC version from {}" , cg_gcc_version_path . display ()) }) . trim () . to_string () ; let git_output = Command :: new ("git") . current_dir (root_path) . arg ("submodule") . arg ("status") . arg ("--cached") . arg ("src/gcc") . output () . expect ("Cannot determine git SHA of the src/gcc checkout") ; if ! git_output . status . success () { eprintln ! ("Cannot figure out the SHA of the GCC submodule") ; return ; } let git_output = String :: from_utf8_lossy (& git_output . stdout) . split_whitespace () . next () . unwrap_or_default () . to_string () ; let gcc_submodule_sha = git_output . trim_start_matches (['+' , '-']) ; if gcc_submodule_sha != cg_gcc_version { * bad = true ; eprintln ! (r#"Commit SHA of the src/gcc submodule (`{gcc_submodule_sha}`) does not match the required GCC version of the GCC codegen backend (`{cg_gcc_version}`).
Make sure to set the src/gcc submodule to commit {cg_gcc_version}.
The GCC codegen backend commit is configured at {}."# , cg_gcc_version_path . display () ,) ; } }
};
}
