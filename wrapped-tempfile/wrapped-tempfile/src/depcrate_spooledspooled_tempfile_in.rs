// Generated macro for spooled_tempfile_in (function)
macro_rules! Depcrate_spooledspooled_tempfile_in {
() => {
// Module: crate::spooled
// Provides: {"spooled_tempfile_in"}
// Dependencies: {}
# [doc = " Construct a new [`SpooledTempFile`], backed by a file in the specified directory. Use this when,"] # [doc = " e.g., you need the temporary file to be backed by a specific filesystem (e.g., when your default"] # [doc = " temporary directory is in-memory). Also see [`spooled_tempfile`]."] # [doc = ""] # [doc = " **NOTE:** The specified path isn't checked until the temporary file is \"rolled over\" into a real"] # [doc = " temporary file. If the specified directory isn't writable, writes to the temporary file will"] # [doc = " fail once the `max_size` is reached."] # [inline] pub fn spooled_tempfile_in < P : AsRef < Path > > (max_size : usize , dir : P) -> SpooledTempFile { SpooledTempFile :: new_in (max_size , dir) }
};
}
