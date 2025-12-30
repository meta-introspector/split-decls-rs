// Generated macro for FDFLAGS_SYNC (const)
macro_rules! Depcrate_lib_generatedFDFLAGS_SYNC {
() => {
// Module: crate::lib_generated
// Provides: {"FDFLAGS_SYNC"}
// Dependencies: {}
# [doc = " Write according to synchronized I/O file integrity completion. In"] # [doc = " addition to synchronizing the data stored in the file, the implementation"] # [doc = " may also synchronously update the file's metadata."] pub const FDFLAGS_SYNC : Fdflags = 1 << 4 ;
};
}
