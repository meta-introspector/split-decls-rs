// Generated macro for ReadDir (struct)
macro_rules! Depcrate_fsReadDir {
() => {
// Module: crate::fs
// Provides: {"ReadDir"}
// Dependencies: {}
# [doc = " Iterator over the entries in a directory."] # [doc = ""] # [doc = " This iterator is returned from the [`read_dir`] function of this module and"] # [doc = " will yield instances of <code>[io::Result]<[DirEntry]></code>. Through a [`DirEntry`]"] # [doc = " information like the entry's path and possibly other metadata can be"] # [doc = " learned."] # [doc = ""] # [doc = " The order in which this iterator returns entries is platform and filesystem"] # [doc = " dependent."] # [doc = ""] # [doc = " # Errors"] # [doc = " This [`io::Result`] will be an [`Err`] if an error occurred while fetching"] # [doc = " the next entry from the OS."] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct ReadDir (fs_imp :: ReadDir) ;
};
}
