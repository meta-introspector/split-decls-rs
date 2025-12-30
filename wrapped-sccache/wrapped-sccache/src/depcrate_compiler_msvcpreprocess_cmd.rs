// Generated macro for preprocess_cmd (function)
macro_rules! Depcrate_compiler_msvcpreprocess_cmd {
() => {
// Module: crate::compiler::msvc
// Provides: {"preprocess_cmd"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] pub fn preprocess_cmd < T > (cmd : & mut T , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , may_dist : bool , rewrite_includes_only : bool , is_clang : bool ,) where T : RunCommand , { if may_dist || parsed_args . profile_generate { cmd . arg ("-E") ; } else { cmd . arg ("-EP") ; } cmd . arg ("-nologo") . args (& parsed_args . preprocessor_args) . args (& parsed_args . dependency_args) . args (& parsed_args . common_args) . env_clear () . envs (env_vars . to_vec ()) . current_dir (cwd) ; if is_clang { if parsed_args . depfile . is_some () && ! parsed_args . msvc_show_includes { cmd . arg ("-showIncludes") ; } } else { if let Some (ref depfile) = parsed_args . depfile { cmd . arg ("/sourceDependencies") ; cmd . arg (depfile) ; } cmd . arg ("/WX-") ; } if rewrite_includes_only && is_clang { cmd . arg ("-clang:-frewrite-includes") ; } if parsed_args . double_dash_input { cmd . arg ("--") ; } cmd . arg (& parsed_args . input) ; }
};
}
