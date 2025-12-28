macro_rules! deps {
    () => {
        VfsPathRepr!();
        Vfs!();
    };
}

macro_rules! VfsPath {
    () => {
        deps!();
        # [doc = " Path in [`Vfs`]."] # [doc = ""] # [doc = " Long-term, we want to support files which do not reside in the file-system,"] # [doc = " so we treat `VfsPath`s as opaque identifiers."] # [doc = ""] # [doc = " [`Vfs`]: crate::Vfs"] # [derive (Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] pub struct VfsPath (VfsPathRepr) ;
    };
}

VfsPath!();