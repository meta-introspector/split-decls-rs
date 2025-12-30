// Generated macro for FileSet (struct)
macro_rules! Depcrate_file_setFileSet {
() => {
// Module: crate::file_set
// Provides: {"FileSet"}
// Dependencies: {}
# [doc = " A set of [`VfsPath`]s identified by [`FileId`]s."] # [derive (Default , Clone , Eq , PartialEq)] pub struct FileSet { files : FxHashMap < VfsPath , FileId > , paths : IndexMap < FileId , VfsPath , FxBuildHasher > , }
};
}
