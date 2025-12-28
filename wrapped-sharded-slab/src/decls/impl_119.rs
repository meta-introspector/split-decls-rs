macro_rules! deps {
    () => {
        TransferStack!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < C > fmt :: Debug for TransferStack < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TransferStack") . field ("head" , & format_args ! ("{:#0x}" , & self . head . load (Ordering :: Relaxed)) ,) . finish () } }
    };
}

impl_119!()