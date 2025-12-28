macro_rules! deps {
    () => {
        VfsPathRepr!();
        VfsPath!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl From < AbsPathBuf > for VfsPath { fn from (v : AbsPathBuf) -> Self { VfsPath (VfsPathRepr :: PathBuf (v . normalize ())) } }
    };
}

impl_36!()