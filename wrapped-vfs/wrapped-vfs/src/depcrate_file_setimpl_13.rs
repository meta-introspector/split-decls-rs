// Generated macro for impl_13 (impl)
macro_rules! Depcrate_file_setimpl_13 {
() => {
// Module: crate::file_set
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Debug for FileSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FileSet") . field ("n_files" , & self . files . len ()) . finish () } }
};
}
