macro_rules! deps {
    () => {
        ArgAbi!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < 'a , Ty : fmt :: Display > fmt :: Debug for ArgAbi < 'a , Ty > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ArgAbi { layout , mode } = self ; f . debug_struct ("ArgAbi") . field ("layout" , layout) . field ("mode" , mode) . finish () } }
    };
}

impl_282!()