macro_rules! deps {
    () => {
        VirtualPath!();
        VfsPathRepr!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl fmt :: Debug for VfsPathRepr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self { VfsPathRepr :: PathBuf (it) => it . fmt (f) , VfsPathRepr :: VirtualPath (VirtualPath (it)) => it . fmt (f) , } } }
    };
}

impl_39!()