macro_rules! deps {
    () => {
        LazyDefPathStr!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Display for LazyDefPathStr < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . tcx . def_path_str (self . def_id)) } }
    };
}

impl_12!();