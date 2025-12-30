// Generated macro for find_sparse_entries (function)
macro_rules! Depcrate_builderfind_sparse_entries {
() => {
// Module: crate::builder
// Provides: {"find_sparse_entries"}
// Dependencies: {}
# [doc = " Find sparse entries in a file. Returns:"] # [doc = " * `Ok(Some(_))` if the file is sparse."] # [doc = " * `Ok(None)` if the file is not sparse, or if the file system does not support sparse files."] # [doc = " * `Err(_)` if an error occurred. The lack of support for sparse files is not"] # [doc = "   considered an error. It might return an error if the file is modified"] # [doc = "   while reading."] fn find_sparse_entries (file : & mut fs :: File , stat : & fs :: Metadata ,) -> io :: Result < Option < SparseEntries > > { # [cfg (not (any (target_os = "android" , target_os = "freebsd" , target_os = "linux")))] { let _ = file ; let _ = stat ; Ok (None) } # [cfg (any (target_os = "android" , target_os = "freebsd" , target_os = "linux"))] find_sparse_entries_seek (file , stat) }
};
}
