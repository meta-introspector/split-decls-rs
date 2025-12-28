macro_rules! deps {
    () => {
        VfsPath!();
        VirtualPath!();
        VfsPathRepr!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl PartialEq < AbsPath > for VfsPath { fn eq (& self , other : & AbsPath) -> bool { match & self . 0 { VfsPathRepr :: PathBuf (lhs) => lhs == other , VfsPathRepr :: VirtualPath (_) => false , } } }
    };
}

impl_40!()