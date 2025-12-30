// Generated macro for PathPersistError (struct)
macro_rules! Depcrate_filePathPersistError {
() => {
// Module: crate::file
// Provides: {"PathPersistError"}
// Dependencies: {}
# [doc = " Error returned when persisting a temporary file path fails."] # [derive (Debug)] pub struct PathPersistError { # [doc = " The underlying IO error."] pub error : io :: Error , # [doc = " The temporary file path that couldn't be persisted."] pub path : TempPath , }
};
}
