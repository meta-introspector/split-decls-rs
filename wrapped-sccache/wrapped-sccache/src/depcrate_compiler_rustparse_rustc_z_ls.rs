// Generated macro for parse_rustc_z_ls (function)
macro_rules! Depcrate_compiler_rustparse_rustc_z_ls {
() => {
// Module: crate::compiler::rust
// Provides: {"parse_rustc_z_ls"}
// Dependencies: {}
# [cfg (feature = "dist-client")] fn parse_rustc_z_ls (stdout : & str) -> Result < Vec < & str > > { let mut lines = stdout . lines () ; loop { match lines . next () { Some ("=External Dependencies=") => break , Some (_s) => { } None => bail ! ("No output from rustc -Z ls") , } } let mut dep_names = vec ! [] ; for line in & mut lines { if line . is_empty () { break ; } let mut line_splits = line . splitn (2 , ' ') ; let num : usize = line_splits . next () . expect ("Zero strings from line split") . parse () . context ("Could not parse number from rustc -Z ls") ? ; let libstring = line_splits . next () . context ("No lib string on line from rustc -Z ls") ? ; if num != dep_names . len () + 1 { bail ! ("Unexpected numbering of {} in rustc -Z ls output" , libstring) } assert ! (line_splits . next () . is_none ()) ; let mut libstring_splits = libstring . rsplitn (2 , '-') ; let libname = { let maybe_hash = libstring_splits . next () . context ("Nothing in lib string from `rustc -Z ls`") ? ; if let Some (name) = libstring_splits . next () { name } else { maybe_hash } } ; assert ! (libstring_splits . next () . is_none ()) ; dep_names . push (libname) ; } for line in lines { if ! line . is_empty () { bail ! ("Trailing non-blank lines in rustc -Z ls output") } } Ok (dep_names) }
};
}
