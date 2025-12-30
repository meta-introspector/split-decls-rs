// Generated macro for SpooledTempFile (struct)
macro_rules! Depcrate_spooledSpooledTempFile {
() => {
// Module: crate::spooled
// Provides: {"SpooledTempFile"}
// Dependencies: {}
# [doc = " An object that behaves like a regular temporary file, but keeps data in"] # [doc = " memory until it reaches a configured size, at which point the data is"] # [doc = " written to a temporary file on disk, and further operations use the file"] # [doc = " on disk."] # [derive (Debug)] pub struct SpooledTempFile { max_size : usize , dir : Option < PathBuf > , inner : SpooledData , }
};
}
