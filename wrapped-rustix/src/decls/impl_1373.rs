macro_rules! deps {
    () => {
        CpuSet!();
        Result!();
    };
}

macro_rules! impl_1373 {
    () => {
        deps!();
        impl fmt :: Debug for CpuSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "CpuSet {{") ? ; let mut first = true ; for i in 0 .. Self :: MAX_CPU { if self . is_set (i) { if first { write ! (f , " ") ? ; first = false ; } else { write ! (f , ", ") ? ; } write ! (f , "cpu{}" , i) ? ; } } write ! (f , " }}") } }
    };
}

impl_1373!()