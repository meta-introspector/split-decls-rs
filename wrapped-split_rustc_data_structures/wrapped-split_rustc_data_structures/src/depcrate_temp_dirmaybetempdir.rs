// Generated macro for MaybeTempDir (struct)
macro_rules! Depcrate_temp_dirMaybeTempDir {
() => {
// Module: crate::temp_dir
// Provides: {"MaybeTempDir"}
// Dependencies: {}
# [doc = " This is used to avoid TempDir being dropped on error paths unintentionally."] # [derive (Debug)] pub struct MaybeTempDir { dir : ManuallyDrop < TempDir > , keep : bool , }
};
}
