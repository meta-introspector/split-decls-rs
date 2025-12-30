// Generated macro for impl_55 (impl)
macro_rules! Depcrate_vfs_pathimpl_55 {
() => {
// Module: crate::vfs_path
// Provides: {"impl_55"}
// Dependencies: {}
impl fmt :: Debug for VfsPathRepr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self { VfsPathRepr :: PathBuf (it) => it . fmt (f) , VfsPathRepr :: VirtualPath (VirtualPath (it)) => it . fmt (f) , } } }
};
}
