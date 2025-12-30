// Generated macro for impl_738 (impl)
macro_rules! Depcrate_fsimpl_738 {
() => {
// Module: crate::fs
// Provides: {"impl_738"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Seek for & File { # [doc = " Seek to an offset, in bytes in a file."] # [doc = ""] # [doc = " See [`Seek::seek`] docs for more info."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " This function currently corresponds to the `lseek64` function on Unix"] # [doc = " and the `SetFilePointerEx` function on Windows. Note that this [may"] # [doc = " change in the future][changes]."] # [doc = ""] # [doc = " [changes]: io#platform-specific-behavior"] fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . inner . seek (pos) } # [doc = " Returns the length of this file (in bytes)."] # [doc = ""] # [doc = " See [`Seek::stream_len`] docs for more info."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " This function currently corresponds to the `statx` function on Linux"] # [doc = " (with fallbacks) and the `GetFileSizeEx` function on Windows. Note that"] # [doc = " this [may change in the future][changes]."] # [doc = ""] # [doc = " [changes]: io#platform-specific-behavior"] fn stream_len (& mut self) -> io :: Result < u64 > { if let Some (result) = self . inner . size () { return result ; } io :: stream_len_default (self) } fn stream_position (& mut self) -> io :: Result < u64 > { self . inner . tell () } }
};
}
