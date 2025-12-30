// Generated macro for TempDir (struct)
macro_rules! DepcrateTempDir {
() => {
// Module: crate
// Provides: {"TempDir"}
// Dependencies: {}
# [doc = " A temporary directory."] # [doc = ""] # [doc = " This is a RAII object which will remove the underlying temporary directory"] # [doc = " when dropped."] # [derive (Debug)] # [must_use] pub struct TempDir { path : PathBuf , }
};
}
