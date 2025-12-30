// Generated macro for IncludeEntry (struct)
macro_rules! Depcrate_compiler_preprocessor_cacheIncludeEntry {
() => {
// Module: crate::compiler::preprocessor_cache
// Provides: {"IncludeEntry"}
// Dependencies: {}
# [doc = " Corresponds to a cached include file used in the pre-processor stage"] # [derive (Deserialize , Serialize , Debug , PartialEq , Eq)] pub struct IncludeEntry { # [doc = " Its absolute path"] path : OsString , # [doc = " The hash of its contents"] digest : String , # [doc = " Its file size, in bytes"] file_size : u64 , # [doc = " Its modification time, `None` if not recorded."] mtime : Option < Timestamp > , # [doc = " Its status change time, `None` if not recorded."] ctime : Option < Timestamp > , }
};
}
