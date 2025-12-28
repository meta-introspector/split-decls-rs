macro_rules! deps {
    () => {
        VirtualPath!();
        VfsPathRepr!();
        VfsPath!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl fmt :: Display for VfsPath { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . 0 { VfsPathRepr :: PathBuf (it) => it . fmt (f) , VfsPathRepr :: VirtualPath (VirtualPath (it)) => it . fmt (f) , } } }
    };
}

impl_37!()