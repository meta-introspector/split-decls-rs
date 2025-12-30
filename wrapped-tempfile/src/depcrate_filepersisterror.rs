// Generated macro for PersistError (struct)
macro_rules! Depcrate_filePersistError {
() => {
// Module: crate::file
// Provides: {"PersistError"}
// Dependencies: {}
# [doc = " Error returned when persisting a temporary file fails."] pub struct PersistError < F = File > { # [doc = " The underlying IO error."] pub error : io :: Error , # [doc = " The temporary file that couldn't be persisted."] pub file : NamedTempFile < F > , }
};
}
