macro_rules! deps {
    () => {
        VirtualPath!();
        VfsPath!();
    };
}

macro_rules! VfsPathRepr {
    () => {
        deps!();
        # [doc = " Internal, private representation of [`VfsPath`]."] # [derive (Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] enum VfsPathRepr { PathBuf (AbsPathBuf) , VirtualPath (VirtualPath) , }
    };
}

VfsPathRepr!()