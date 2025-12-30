// Generated macro for normalize_path (function)
macro_rules! Depcrate_compiler_cnormalize_path {
() => {
// Module: crate::compiler::c
// Provides: {"normalize_path"}
// Dependencies: {}
# [doc = " Copied from cargo."] # [doc = ""] # [doc = " Normalize a path, removing things like `.` and `..`."] # [doc = ""] # [doc = " CAUTION: This does not resolve symlinks (unlike"] # [doc = " [`std::fs::canonicalize`])."] pub fn normalize_path (path : & Path) -> PathBuf { use std :: path :: Component ; let mut components = path . components () . peekable () ; let mut ret = if let Some (c @ Component :: Prefix (..)) = components . peek () . cloned () { components . next () ; PathBuf :: from (c . as_os_str ()) } else { PathBuf :: new () } ; for component in components { match component { Component :: Prefix (..) => unreachable ! () , Component :: RootDir => { ret . push (component . as_os_str ()) ; } Component :: CurDir => { } Component :: ParentDir => { ret . pop () ; } Component :: Normal (c) => { ret . push (c) ; } } } ret }
};
}
