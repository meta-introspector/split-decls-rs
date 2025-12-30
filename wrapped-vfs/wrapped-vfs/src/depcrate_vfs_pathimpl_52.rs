// Generated macro for impl_52 (impl)
macro_rules! Depcrate_vfs_pathimpl_52 {
() => {
// Module: crate::vfs_path
// Provides: {"impl_52"}
// Dependencies: {}
impl From < AbsPathBuf > for VfsPath { fn from (v : AbsPathBuf) -> Self { VfsPath (VfsPathRepr :: PathBuf (v . normalize ())) } }
};
}
