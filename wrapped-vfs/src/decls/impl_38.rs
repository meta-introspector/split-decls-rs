macro_rules! deps {
    () => {
        VfsPath!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl fmt :: Debug for VfsPath { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_38!();