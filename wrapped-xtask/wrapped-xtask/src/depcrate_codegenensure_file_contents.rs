// Generated macro for ensure_file_contents (function)
macro_rules! Depcrate_codegenensure_file_contents {
() => {
// Module: crate::codegen
// Provides: {"ensure_file_contents"}
// Dependencies: {}
# [doc = " Checks that the `file` has the specified `contents`. If that is not the"] # [doc = " case, updates the file and then fails the test."] # [allow (clippy :: print_stderr)] fn ensure_file_contents (cg : CodegenType , file : & Path , contents : & str , check : bool) -> bool { let contents = normalize_newlines (contents) ; if let Ok (old_contents) = fs :: read_to_string (file) && normalize_newlines (& old_contents) == contents { return false ; } let display_path = file . strip_prefix (project_root ()) . unwrap_or (file) ; if check { panic ! ("{} was not up-to-date{}" , file . display () , if std :: env :: var ("CI") . is_ok () { format ! ("\n    NOTE: run `cargo xtask codegen {cg}` locally and commit the updated files\n") } else { "" . to_owned () }) ; } else { eprintln ! ("\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n" , display_path . display ()) ; if let Some (parent) = file . parent () { let _ = fs :: create_dir_all (parent) ; } fs :: write (file , contents) . unwrap () ; true } }
};
}
