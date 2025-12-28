macro_rules! impl_94 {
    () => {
        impl fmt :: Display for CrateNum { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . as_u32 () , f) } }
    };
}

impl_94!();