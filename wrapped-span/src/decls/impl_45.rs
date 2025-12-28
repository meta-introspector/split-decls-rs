macro_rules! deps {
    () => {
        AstIdMap!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl fmt :: Debug for AstIdMap { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AstIdMap") . field ("arena" , & self . arena) . finish () } }
    };
}

impl_45!()