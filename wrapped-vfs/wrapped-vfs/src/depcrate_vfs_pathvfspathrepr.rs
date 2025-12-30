// Generated macro for VfsPathRepr (enum)
macro_rules! Depcrate_vfs_pathVfsPathRepr {
() => {
// Module: crate::vfs_path
// Provides: {"VfsPathRepr"}
// Dependencies: {}
# [doc = " Internal, private representation of [`VfsPath`]."] # [derive (Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] enum VfsPathRepr { PathBuf (AbsPathBuf) , VirtualPath (VirtualPath) , }
};
}
