// Generated macro for copy_path_into_gnu_long (function)
macro_rules! Depcrate_headercopy_path_into_gnu_long {
() => {
// Module: crate::header
// Provides: {"copy_path_into_gnu_long"}
// Dependencies: {}
# [doc = " Copies `path` into the `slot` provided"] # [doc = ""] # [doc = " Returns an error if:"] # [doc = ""] # [doc = " * the path is too long to fit"] # [doc = " * a nul byte was found"] # [doc = " * an invalid path component is encountered (e.g. a root path or parent dir)"] # [doc = " * the path itself is empty"] # [doc = ""] # [doc = " This is less restrictive version meant to be used for truncated GNU paths."] fn copy_path_into_gnu_long (slot : & mut [u8] , path : & Path , is_link_name : bool) -> io :: Result < () > { copy_path_into_inner (slot , path , is_link_name , true) }
};
}
