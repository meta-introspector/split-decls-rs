// Generated macro for include_is_too_new (function)
macro_rules! Depcrate_compiler_cinclude_is_too_new {
() => {
// Module: crate::compiler::c
// Provides: {"include_is_too_new"}
// Dependencies: {}
# [doc = " Opt out of preprocessor cache mode because of a race condition."] # [doc = ""] # [doc = " The race condition consists of these events:"] # [doc = ""] # [doc = " - the preprocessor is run"] # [doc = " - an include file is modified by someone"] # [doc = " - the new include file is hashed by sccache"] # [doc = " - the real compiler is run on the preprocessor's output, which contains"] # [doc = "   data from the old header file"] # [doc = " - the wrong object file is stored in the cache."] fn include_is_too_new (path : & Path , meta : & PreprocessorFileMetadata , time_of_compilation : std :: time :: SystemTime ,) -> bool { if let Some (mtime) = meta . modified { if mtime >= time_of_compilation . into () { debug ! ("Include file {} is too new" , path . display ()) ; return true ; } } if let Some (ctime) = meta . ctime_or_creation { if ctime >= time_of_compilation . into () { debug ! ("Include file {} is too new" , path . display ()) ; return true ; } } false }
};
}
