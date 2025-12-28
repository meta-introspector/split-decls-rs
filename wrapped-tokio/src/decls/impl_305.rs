macro_rules! deps {
    () => {
        LinkedList!();
        Link!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < L : Link > fmt :: Debug for LinkedList < L , L :: Target > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("LinkedList") . field ("head" , & self . head) . field ("tail" , & self . tail) . finish () } }
    };
}

impl_305!()