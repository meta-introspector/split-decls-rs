macro_rules! deps {
    () => {
        FileId!();
        VfsPath!();
    };
}

macro_rules! PathInterner {
    () => {
        deps!();
        # [doc = " Structure to map between [`VfsPath`] and [`FileId`]."] # [derive (Default)] pub (crate) struct PathInterner { map : IndexSet < VfsPath , BuildHasherDefault < FxHasher > > , }
    };
}

PathInterner!();