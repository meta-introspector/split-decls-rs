macro_rules! deps {
    () => {
        VfsPath!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl PartialEq < VfsPath > for AbsPath { fn eq (& self , other : & VfsPath) -> bool { other == self } }
    };
}

impl_41!();