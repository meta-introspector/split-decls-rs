// Generated macro for Entry (enum)
macro_rules! Depcrate_loaderEntry {
() => {
// Module: crate::loader
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A set of files on the file system."] # [derive (Debug , Clone)] pub enum Entry { # [doc = " The `Entry` is represented by a raw set of files."] Files (Vec < AbsPathBuf >) , # [doc = " The `Entry` is represented by `Directories`."] Directories (Directories) , }
};
}
