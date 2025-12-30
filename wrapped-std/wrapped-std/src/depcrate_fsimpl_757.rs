// Generated macro for impl_757 (impl)
macro_rules! Depcrate_fsimpl_757 {
() => {
// Module: crate::fs
// Provides: {"impl_757"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl fmt :: Debug for FileType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FileType") . field ("is_file" , & self . is_file ()) . field ("is_dir" , & self . is_dir ()) . field ("is_symlink" , & self . is_symlink ()) . finish_non_exhaustive () } }
};
}
