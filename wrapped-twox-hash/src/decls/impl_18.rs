macro_rules! deps {
    () => {
        Accumulators!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl fmt :: Debug for Accumulators { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let [acc1 , acc2 , acc3 , acc4] = self . 0 ; f . debug_struct ("Accumulators") . field ("acc1" , & acc1) . field ("acc2" , & acc2) . field ("acc3" , & acc3) . field ("acc4" , & acc4) . finish () } }
    };
}

impl_18!();