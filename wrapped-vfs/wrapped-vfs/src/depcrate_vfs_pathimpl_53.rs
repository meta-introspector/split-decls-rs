// Generated macro for impl_53 (impl)
macro_rules! Depcrate_vfs_pathimpl_53 {
() => {
// Module: crate::vfs_path
// Provides: {"impl_53"}
// Dependencies: {}
impl fmt :: Display for VfsPath { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . 0 { VfsPathRepr :: PathBuf (it) => it . fmt (f) , VfsPathRepr :: VirtualPath (VirtualPath (it)) => it . fmt (f) , } } }
};
}
