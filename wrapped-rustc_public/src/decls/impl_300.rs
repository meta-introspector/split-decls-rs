macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl Debug for Ty { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ty") . field ("id" , & self . 0) . field ("kind" , & self . kind ()) . finish () } }
    };
}

impl_300!()