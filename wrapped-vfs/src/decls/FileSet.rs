macro_rules! deps {
    () => {
        VfsPath!();
        FileId!();
    };
}

macro_rules! FileSet {
    () => {
        deps!();
        # [doc = " A set of [`VfsPath`]s identified by [`FileId`]s."] # [derive (Default , Clone , Eq , PartialEq)] pub struct FileSet { files : FxHashMap < VfsPath , FileId > , paths : IndexMap < FileId , VfsPath , FxBuildHasher > , }
    };
}

FileSet!()