macro_rules! deps {
    () => {
        FileSet!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl fmt :: Debug for FileSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FileSet") . field ("n_files" , & self . files . len ()) . finish () } }
    };
}

impl_5!()