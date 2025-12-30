// Generated macro for dir_entry_is_file (function)
macro_rules! Depcrate_unstable_bookdir_entry_is_file {
() => {
// Module: crate::unstable_book
// Provides: {"dir_entry_is_file"}
// Dependencies: {}
# [doc = " Tests whether `DirEntry` is a file."] fn dir_entry_is_file (dir_entry : & fs :: DirEntry) -> bool { dir_entry . file_type () . expect ("could not determine file type of directory entry") . is_file () }
};
}
