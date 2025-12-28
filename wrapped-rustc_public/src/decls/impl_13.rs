macro_rules! impl_13 {
    () => {
        impl Debug for DefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("id" , & self . 0) . field ("name" , & self . name ()) . finish () } }
    };
}

impl_13!()