// Generated macro for impl_56 (impl)
macro_rules! Depcrate_vfs_pathimpl_56 {
() => {
// Module: crate::vfs_path
// Provides: {"impl_56"}
// Dependencies: {}
impl PartialEq < AbsPath > for VfsPath { fn eq (& self , other : & AbsPath) -> bool { match & self . 0 { VfsPathRepr :: PathBuf (lhs) => lhs == other , VfsPathRepr :: VirtualPath (_) => false , } } }
};
}
