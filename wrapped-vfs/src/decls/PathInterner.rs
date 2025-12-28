macro_rules! deps {
    () => {
        VfsPath!();
        FileId!();
    };
}

macro_rules! PathInterner {
    () => {
        deps!();
        # [doc = " Structure to map between [`VfsPath`] and [`FileId`]."] # [derive (Default)] pub (crate) struct PathInterner { map : IndexSet < VfsPath , BuildHasherDefault < FxHasher > > , }
    };
}

PathInterner!()