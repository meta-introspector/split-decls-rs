// Generated macro for copy_path_into (function)
macro_rules! Depcrate_headercopy_path_into {
() => {
// Module: crate::header
// Provides: {"copy_path_into"}
// Dependencies: {}
# [doc = " Copies `path` into the `slot` provided"] # [doc = ""] # [doc = " Returns an error if:"] # [doc = ""] # [doc = " * the path is too long to fit"] # [doc = " * a nul byte was found"] # [doc = " * an invalid path component is encountered (e.g. a root path or parent dir)"] # [doc = " * the path itself is empty"] fn copy_path_into (slot : & mut [u8] , path : & Path , is_link_name : bool) -> io :: Result < () > { copy_path_into_inner (slot , path , is_link_name , false) }
};
}
